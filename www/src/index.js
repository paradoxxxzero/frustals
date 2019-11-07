import { GUI } from "dat.gui";
import debounce from "debounce";
import { Frustal, Variant, Point, Colorization } from "frustals";

import { memory } from "frustals/frustals_bg";
import presets from "./presets";
import "./index.sass";

const defaultFractal = "Newton z⁶ + z³ - 1";

const fractalLabels = {
  BurningShip: "Burning Ship",
  Newton: "Newton z³ - 1",
  Newton2: "Newton z³ - 2z + 2",
  Newton3: "Newton z⁶ + z³ - 1",
  Newton4: "Newton z⁵ - 2",
  Newton5: "Newton z³ - 1 + 1/z"
};

const jsOptions = {
  preview: true,
  previewScale: 10
};

const mainCanvas = document.createElement("canvas");
const previewCanvas = document.createElement("canvas");
const { width, height } = document.body.getBoundingClientRect();

previewCanvas.width = width / jsOptions.previewScale;
previewCanvas.height = height / jsOptions.previewScale;
previewCanvas.classList.add("frustal-canvas");
previewCanvas.classList.add("frustal-preview");
document.body.appendChild(previewCanvas);

mainCanvas.width = width;
mainCanvas.height = height;
mainCanvas.classList.add("frustal-canvas");
document.body.appendChild(mainCanvas);

const frustal = Frustal.new(width, height, jsOptions.previewScale);
let started = false;
let renderId = 0;

const draw = (canvas, ptr) => {
  canvas
    .getContext("2d")
    .putImageData(
      new ImageData(
        new Uint8ClampedArray(
          memory.buffer,
          ptr,
          canvas.width * canvas.height * 4
        ),
        canvas.width,
        canvas.height
      ),
      0,
      0
    );
};

const render = async () => {
  if (!started) {
    return;
  }
  const id = ++renderId;
  let interactionDelay = 0;
  if (jsOptions.preview) {
    mainCanvas
      .getContext("2d")
      .clearRect(0, 0, mainCanvas.width, mainCanvas.height);
    const t0 = performance.now();
    await frustal.preview_render();
    const t1 = performance.now();
    console.log(`Preview render : ${t1 - t0}ms. Drawn`);
    draw(previewCanvas, frustal.preview_data_ptr());
    interactionDelay = t1 - t0 + 50;
  }
  setTimeout(async () => {
    if (id !== renderId) {
      return;
    }
    frustal.reset_data();
    const splits = 16;
    let i = 0;
    while (i++ < splits) {
      const t0 = performance.now();
      await frustal.partial_render(splits, i - 1);
      const t1 = performance.now();
      if (id === renderId) {
        console.log(`Render ${i}/${splits} : ${t1 - t0}ms. Drawn`);
        draw(mainCanvas, frustal.data_ptr());
        await new Promise(resolve => setTimeout(resolve, 1));
      } else {
        console.log(`Render ${i}/${splits} : ${t1 - t0}ms. Not drawn`);
        return;
      }
    }
  }, interactionDelay);
};

window.addEventListener(
  "resize",
  debounce(() => {
    const { width, height } = document.body.getBoundingClientRect();
    mainCanvas.width = width;
    mainCanvas.height = height;
    previewCanvas.width = Math.floor(width / jsOptions.previewScale);
    previewCanvas.height = Math.floor(height / jsOptions.previewScale);
    frustal.resize(width, height);
    render();
  }, 10),
  false
);

// window.addEventListener(
//   "keydown",
//   ({ keyCode }) => {
//     switch (keyCode) {
//       case 37: // left
//         frustal.shift_domain(Point.new(-50, 0));
//         break;
//       case 38: // up
//         frustal.shift_domain(Point.new(0, -50));
//         break;
//       case 39: // right
//         frustal.shift_domain(Point.new(50, 0));
//         break;
//       case 40: // down
//         frustal.shift_domain(Point.new(0, 50));
//         break;
//       case 187: // +
//         frustal.scale_domain(0.75);
//         break;
//       case 189: // -
//         frustal.scale_domain(1.5);
//         break;
//       default:
//         return;
//     }
//     updateDomain();
//     render();
//   },
//   false
// );

const drag = {
  handler: null,
  x: null,
  y: null,
  x2: null,
  y2: null
};

mainCanvas.addEventListener(
  "mousedown",
  ({ clientX, clientY }) => {
    drag.x = clientX;
    drag.y = clientY;
    mainCanvas.addEventListener(
      "mousemove",
      (drag.handler = debounce(({ clientX, clientY }) => {
        if (!drag.handler || (drag.x === clientX && drag.y === clientY)) {
          return;
        }
        frustal.shift_domain(Point.new(drag.x - clientX, drag.y - clientY));
        updateDomain();
        render();
        drag.x = clientX;
        drag.y = clientY;
      }, 1)),
      false
    );
  },
  false
);
mainCanvas.addEventListener(
  "touchstart",
  ({ touches }) => {
    drag.x = touches[0].clientX;
    drag.y = touches[0].clientY;
    if (touches.length > 1) {
      drag.x2 = touches[1].clientX;
      drag.y2 = touches[1].clientY;
    }

    mainCanvas.addEventListener(
      "touchmove",
      (drag.handler = debounce(({ touches }) => {
        if (!drag.handler) {
          return;
        }
        let hasZoomed = false;
        if (touches.length > 1) {
          if (!drag.x2) {
            drag.x2 = touches[1].clientX;
            drag.y2 = touches[1].clientY;
          } else {
            const x = touches[0].clientX;
            const y = touches[0].clientY;
            const x2 = touches[1].clientX;
            const y2 = touches[1].clientY;

            const xc = (x + x2 + drag.x + drag.x2) / 4;
            const yc = (y + y2 + drag.y + drag.y2) / 4;
            const previousDistance = Math.sqrt(
              (drag.x2 - drag.x) * (drag.x2 - drag.x) +
                (drag.y2 - drag.y) * (drag.y2 - drag.y)
            );
            const currentDistance = Math.sqrt(
              (x2 - x) * (x2 - x) + (y2 - y) * (y2 - y)
            );
            frustal.zoom_domain(
              previousDistance - currentDistance,
              Point.new(xc, yc)
            );
            updateDomain();
            render();

            drag.x = x;
            drag.y = y;
            drag.x2 = x2;
            drag.y2 = y2;
            hasZoomed = true;
          }
        } else {
          drag.x2 = null;
          drag.y2 = null;
        }
        if (!hasZoomed) {
          const [{ clientX, clientY }] = touches;
          frustal.shift_domain(Point.new(drag.x - clientX, drag.y - clientY));
          updateDomain();
          render();
          drag.x = clientX;
          drag.y = clientY;
        }
      }, 1)),
      false
    );
  },
  false
);

window.addEventListener(
  "mouseup",
  () => {
    if (drag.handler) {
      mainCanvas.removeEventListener("mousemove", drag.handler);
      mainCanvas.removeEventListener("touchmove", drag.handler);
      drag.handler = drag.x = drag.x2 = drag.y = drag.y2 = null;
    }
  },
  false
);
window.addEventListener(
  "touchend",
  () => {
    if (drag.handler) {
      mainCanvas.removeEventListener("mousemove", drag.handler);
      mainCanvas.removeEventListener("touchmove", drag.handler);
      drag.handler = drag.x = drag.x2 = drag.y = drag.y2 = null;
    }
  },
  false
);

mainCanvas.addEventListener(
  "wheel",
  ({ deltaY, clientX, clientY }) => {
    const { height } = document.body.getBoundingClientRect();
    frustal.zoom_domain(deltaY, Point.new(clientX, clientY));
    updateDomain();
    render();
  },
  false
);
const options = frustal.options;

const sync = debounce(() => {
  frustal.sync_options(options);
  render();
}, 25);

const gui = new GUI({
  load: presets,
  preset: decodeURIComponent(location.hash.replace(/^#/, "")) || defaultFractal
});
gui
  .add(
    options,
    "variant",
    Object.entries(Variant).reduce((acc, [name, index]) => {
      acc[fractalLabels[name] || name] = index;
      return acc;
    }, {})
  )
  .onChange(sync);
gui.add(options, "precision", 2).onChange(sync);
gui.add(options, "smooth").onChange(sync);
gui.add(options, "order", 1, 15).onChange(sync);
gui
  .add(options, "const_real", -1.0, 1.0)
  .step(0.01)
  .onChange(sync);
gui
  .add(options, "const_imaginary", -1.0, 1.0)
  .step(0.01)
  .onChange(sync);
gui.add(options, "colorization", Colorization).onChange(sync);
gui.add(options, "lightness", 0, 10.0).onChange(sync);

const { origin, scale } = frustal.current_domain();
const view = {
  x: origin.x,
  y: origin.y,
  scale: scale
};

const updateDomain = () => {
  const { origin, scale } = frustal.current_domain();
  view.x = origin.x;
  view.y = origin.y;
  view.scale = scale;
  gui.__controllers.map(c => c.updateDisplay());
};

gui.remember(options);
const syncDomain = debounce((...args) => {
  frustal.change_domain(view.x, view.y, view.scale);
  render();
}, 25);

gui
  .add(view, "x")
  .step(0.000001)
  .onChange(syncDomain);
gui
  .add(view, "y")
  .step(0.000001)
  .onChange(syncDomain);
gui
  .add(view, "scale")
  .min(0)
  .step(0.000001)
  .onChange(syncDomain);

gui.remember(view);

gui.add(jsOptions, "preview").onChange(() => {
  previewCanvas
    .getContext("2d")
    .clearRect(0, 0, previewCanvas.width, previewCanvas.height);
});

gui
  .add(jsOptions, "previewScale")
  .min(1)
  .max(20)
  .step(1)
  .onChange(
    debounce(() => {
      previewCanvas.width = Math.floor(
        mainCanvas.width / jsOptions.previewScale
      );
      previewCanvas.height = Math.floor(
        mainCanvas.height / jsOptions.previewScale
      );
      frustal.resize_preview(jsOptions.previewScale);
    }, 10)
  );

gui.remember(jsOptions);

gui.revert();
gui.__preset_select.addEventListener("change", ({ target: { value } }) => {
  location.hash = `#${encodeURIComponent(value)}`;
});
window.addEventListener("hashchange", event => {
  gui.preset =
    decodeURIComponent(location.hash.replace(/^#/, "")) || defaultFractal;
  gui.revert();
});

setTimeout(() => {
  started = true;
  render();
}, 30);

window.frustal = frustal;
window.Variant = Variant;
window.Point = Point;
window.render = render;
window.updateDomain = updateDomain;
window.gui = gui;
