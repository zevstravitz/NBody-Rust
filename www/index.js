import { memory } from "nbody-physics/physics_bg";
import * as THREE from 'three';
import { Simulator } from "nbody-physics";

const NUM_PARTICLES = 100;
// const canvas = document.getElementById("nbody-canvas");
let simulator = Simulator.new(NUM_PARTICLES);

let camera, scene, renderer, material, particles;
let mouseX = 0, mouseY = 0;

let windowHalfX = window.innerWidth / 2;
let windowHalfY = window.innerHeight / 2;

init();
animate();

function init() {

    camera = new THREE.PerspectiveCamera( 45, window.innerWidth / window.innerHeight, 2, 5000 );
    camera.position.z = 3000;


    scene = new THREE.Scene();
    // scene.fog = new THREE.FogExp2( 0x000000, 0.0001 );

    const geometry = new THREE.BufferGeometry();
    const vertices = [];

    const sprite = new THREE.TextureLoader().load( 'disc.png' );

    for ( let i = 0; i < NUM_PARTICLES; i ++ ) {

        const x = simulator.get_dim(i, 0);
        const y = simulator.get_dim(i, 1);
        const z = simulator.get_dim(i, 2);

        vertices.push( x, y, z );
    }

    // for ( let i = 0; i < NUM_PARTICLES; i ++ ) {
    //     console.log(simulator.get_dim(i, 0), simulator.get_dim(i, 1), simulator.get_dim(i, 2));
    // }

    geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( vertices, 3 ) );

    material = new THREE.PointsMaterial( { size: 40, sizeAttenuation: true, map: sprite, alphaTest: 0.5, transparent: true } );
    material.color.setHSL( 225.0, 0.73, 0.93 );

    particles = new THREE.Points( geometry, material );
    scene.add( particles );

    //

    renderer = new THREE.WebGLRenderer();
    renderer.setPixelRatio( window.devicePixelRatio );
    renderer.setSize( window.innerWidth, window.innerHeight );
    document.body.appendChild( renderer.domElement );


    document.body.style.touchAction = 'none';
    document.body.addEventListener( 'pointermove', onPointerMove );

    window.addEventListener( 'resize', onWindowResize );

}

function onWindowResize() {

    windowHalfX = window.innerWidth / 2;
    windowHalfY = window.innerHeight / 2;

    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize( window.innerWidth, window.innerHeight );

}

function onPointerMove( event ) {
    if ( event.isPrimary === false ) return;

    mouseX = event.clientX - windowHalfX;
    mouseY = event.clientY - windowHalfY;
}

//

function animate() {

    requestAnimationFrame( animate );

    render();
}

async function render() {

    const time = Date.now() * 0.00005;
    camera.position.x = 500.0 - mouseX;
    camera.position.y = 500.0 + mouseY;

    camera.lookAt( scene.position );

    const positions = particles.geometry.attributes.position.array;

    for (let i = 0; i < NUM_PARTICLES - 3; i+=3) {
        positions[i] = simulator.get_dim(i, 0)
        positions[i+1] = simulator.get_dim(i, 1)
        positions[i+2] = simulator.get_dim(i, 2)
    }

    simulator.next_state();
    await sleep(5000);
    // console.log(simulator.get_dim(0, 0));

    particles.geometry.attributes.position.needsUpdate = true;

    // const h = ( 360 * ( 1.0 + time ) % 360 ) / 360;
    // material.color.setHSL( h, 0.5, 0.5 );

    renderer.render( scene, camera );

}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

// import { Simulator } from "nbody-physics";
//
//
// const NUM_PARTICLES = 50;
// const canvas = document.getElementById("nbody-canvas");
// let simulator = Simulator.new(NUM_PARTICLES);
//
// const ctx = canvas.getContext("2d");
//
// function sleep(ms) {
//     return new Promise(resolve => setTimeout(resolve, ms));
// }
//
// const renderLoop = async () => {
//     drawBodies();
//     await sleep(5000);
//     simulator.next_state();
//     requestAnimationFrame(renderLoop);
// }
//
// const drawBodies = () => {
//     // We want to go into the buffer allocated for WASM with an offset of
//     ctx.clearRect(0,0, canvas.width, canvas.height);
//     for (let i = 0; i < NUM_PARTICLES; i++) {
//         console.log(simulator.get_x(i), simulator.get_y(i));
//         ctx.beginPath();
//         ctx.arc(
//             simulator.get_x(i),
//             simulator.get_y(i),
//             3,
//             0,
//             2 * Math.PI,
//             false
//         );
//         ctx.fillStyle = 'blue';
//         ctx.fill();
//     }
// }
//
// const main = () => {
//     renderLoop();
// }
//
// main();



