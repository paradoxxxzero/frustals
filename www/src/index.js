import { Frustal, FractalType } from "frustals";
import { memory } from "frustals/frustals_bg";
import "./index.sass";

const canvas = document.createElement("canvas");
const ctx = canvas.getContext("2d");
const frustal = Frustal.new(canvas.width, canvas.height);

const render = async () => {
  const t0 = performance.now();
  ctx.putImageData(
    new ImageData(
      new Uint8ClampedArray(
        memory.buffer,
        await frustal.render(),
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
    console.log(keyCode);
    switch (keyCode) {
      case 78:
        frustal.set_type(FractalType.Newton);
        render();
        break;
      case 77:
        frustal.set_type(FractalType.Mandelbrot);
        render();
        break;
    }
  },
  false
);
resize();

window.render = render;
window.frustal = frustal;
