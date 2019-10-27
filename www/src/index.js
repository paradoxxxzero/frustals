import { GUI } from "dat.gui";
import debounce from "debounce";
import { Frustal, Variant } from "frustals";
import { memory } from "frustals/frustals_bg";
import presets from "./presets";
import "./index.sass";

const canvas = document.createElement("canvas");
const ctx = canvas.getContext("2d");
const frustal = Frustal.new(canvas.width, canvas.height);

const render = async () => {
  const t0 = performance.now();
  frustal.render();
  ctx.putImageData(
    new ImageData(
      new Uint8ClampedArray(
        memory.buffer,
        frustal.data_ptr(),
        canvas.width * canvas.height * 4
      ),
      canvas.width,
      canvas.height
    ),
    0,
    0
  );
  const t1 = performance.now();
  console.log("Render " + (t1 - t0) + " ms.");
};

const resize = () => {
  const { width, height } = document.body.getBoundingClientRect();
  canvas.width = width;
  canvas.height = height;
  frustal.resize(width, height);
  render();
};

canvas.classList.add("frustal-canvas");
document.body.appendChild(canvas);
window.addEventListener("resize", resize, false);
window.addEventListener(
  "keydown",
  ({ keyCode }) => {
    switch (keyCode) {
      case 78:
        frustal.set_type(Variant.Newton);
        render();
        break;
      case 77:
        frustal.set_type(Variant.Mandelbrot);
        render();
        break;
    }
  },
  false
);
resize();

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

gui.remember(options);
window.frustal = frustal;
