import { GUI } from "dat.gui";
import debounce from "debounce";
import { Frustal, Variant, Point } from "frustals";
import { memory } from "frustals/frustals_bg";
import presets from "./presets";
import "./index.sass";

const canvas = document.createElement("canvas");
const ctx = canvas.getContext("2d");
const { width, height } = document.body.getBoundingClientRect();
canvas.width = width;
canvas.height = height;
const frustal = Frustal.new(width, height);
let dataPtr = frustal.data_ptr();

let renderId = 0;

const draw = () => {
  ctx.putImageData(
    new ImageData(
      new Uint8ClampedArray(
        memory.buffer,
        dataPtr,
        canvas.width * canvas.height * 4
      ),
      canvas.width,
      canvas.height
    ),
    0,
    0
  );
};

// const _render = async (skip, index) => {
//   const t0 = performance.now();
//
//   const t1 = performance.now();
//   console.log(`Render ${index}/${skip} : ${t1 - t0}ms.`);
// };

const render = async () => {
  const id = ++renderId;
  const splits = 16;
  let i = 0;
  while (i++ < splits) {
    await frustal.partial_render(splits, i);
    if (id === renderId) {
      draw();
      await new Promise(resolve => setTimeout(resolve, 1));
    } else {
      return;
    }
  }
};

canvas.classList.add("frustal-canvas");
document.body.appendChild(canvas);

window.addEventListener(
  "resize",
  debounce(() => {
    const { width, height } = document.body.getBoundingClientRect();
    canvas.width = width;
    canvas.height = height;
    frustal.resize(width, height);
    dataPtr = frustal.data_ptr();
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
  dragging: false
};

canvas.addEventListener(
  "mousedown",
  ({ clientX, clientY }) => {
    drag.dragging = true;
    drag.x = clientX;
    drag.y = clientY;
  },
  false
);

canvas.addEventListener(
  "mousemove",
  debounce(({ clientX, clientY }) => {
    if (!drag.dragging) {
      return;
    }
    frustal.shift_domain(Point.new(drag.x - clientX, clientY - drag.y));
    updateDomain();
    render();
    drag.x = clientX;
    drag.y = clientY;
  }, 1),
  false
);

canvas.addEventListener(
  "mouseup",
  () => {
    drag.dragging = false;
  },
  false
);

canvas.addEventListener(
  "wheel",
  ({ deltaY, clientX, clientY }) => {
    const { height } = document.body.getBoundingClientRect();
    frustal.zoom_domain(deltaY, Point.new(clientX, height - clientY));
    updateDomain();
    render();
  },
  false
);
const options = frustal.current_options();

const sync = debounce(() => {
  frustal.sync_options(options);
  render();
}, 25);

const gui = new GUI({
  load: presets,
  preset: "Mandelbrot"
});
gui.add(options, "variant", Variant).onChange(sync);
gui.add(options, "precision", 2).onChange(sync);
gui.add(options, "smooth").onChange(sync);
gui.add(options, "order", 1, 15).onChange(sync);
gui.add(options, "julia_real", -1.0, 1.0).onChange(sync);
gui.add(options, "julia_imaginary", -1.0, 1.0).onChange(sync);
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
render();
window.frustal = frustal;
window.Point = Point;
window.render = render;
window.updateDomain = updateDomain;
