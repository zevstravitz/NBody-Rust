import { Simulator, Particle } from "nbody-physics";
import { memory } from "nbody-physics/physics_bg";

const NUM_PARTICLES = 4;
const canvas = document.getElementById("nbody-canvas");
let simulator = Simulator.new(NUM_PARTICLES);

const ctx = canvas.getContext("2d");

const renderLoop = () => {
    const particlesPtr = simulator.particles();
    const particleSize = simulator.particle_size();
    const particles = new Uint8Array(
        memory.buffer,
        particlesPtr,
        NUM_PARTICLES * particleSize
    );

    // We want to go into the buffer allocated for WASM with an offset of
    // index * PARTICLE_STRUCT_SIZE
    for (let i = 0; i < NUM_PARTICLES; i++) {
        ctx.beginPath();
        ctx.arc(
            particles[i*particleSize].get_x(),
            particles[i*particleSize].get_y(),
            10,
            0,
            2 * Math.PI,
            false
        );
        ctx.fillStyle = 'green';
        ctx.fill();
    }
    simulator.next_state();

    requestAnimationFrame(renderLoop);
}

const main = () => {
    renderLoop();
}

main();

// context.arc(centerX, centerY, radius, 0, 2 * Math.PI, false);
// context.fillStyle = 'green';
// context.fil



