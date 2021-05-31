import { Simulator, Particle } from "nbody-physics";
// import { memory } from "nbody-physics/physics_bg";

const NUM_PARTICLES = 4;
const canvas = document.getElementById("nbody-canvas");
canvas.width = 800;
canvas.height = 600;
let simulator = Simulator.new(NUM_PARTICLES);

const ctx = canvas.getContext("2d");

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const renderLoop = async () => {
    await drawBodies();
    simulator.next_state();
    requestAnimationFrame(renderLoop);
}

const drawBodies = async () => {
    // We want to go into the buffer allocated for WASM with an offset of
    ctx.clearRect(0,0, canvas.width, canvas.height);
    // await sleep(2000);
    for (let i = 0; i < NUM_PARTICLES; i++) {
        console.log("Updating particle ", i);
        // console.log(canvas.width, canvas.height);
        ctx.beginPath();
        ctx.arc(
            simulator.get_x(i),
            simulator.get_y(i),
            10,
            0,
            2 * Math.PI,
            false
        );
        ctx.fillStyle = 'green';
        ctx.fill();
    }
}

const main = () => {
    renderLoop();
}

main();

// context.arc(centerX, centerY, radius, 0, 2 * Math.PI, false);
// context.fillStyle = 'green';
// context.fil



