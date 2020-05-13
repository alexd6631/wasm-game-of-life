import {Universe, Renderer, create_renderer} from "game-of-life";

//wasm.greet("Alex");

//alert("Factorial 10 = " + wasm.factorial(BigInt(10)));

const canvas = document.getElementById("game-of-life-canvas");
const renderer = create_renderer(canvas.width, canvas.height, canvas);
const renderLoop = () => {
    renderer.draw();
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
