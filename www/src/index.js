import { Frustal } from "frustals";
import { memory } from "frustals/frustals_bg";
import "./index.sass";

const canvas = document.createElement("canvas");
const ctx = canvas.getContext("2d");

const render = () => {
  ctx.fillRect(0, 0, 100, 100);
};

const resize = () => {
  const { width, height } = document.body.getBoundingClientRect();
  canvas.width = width;
  canvas.height = height;
  render();
};

canvas.classList.add("frustal-canvas");
document.body.appendChild(canvas);
window.addEventListener("resize", resize, false);
resize();

// const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

const frustal = Frustal.new(canvas.width, canvas.height);
frustal.render();
const dataPtr = frustal.data();
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
