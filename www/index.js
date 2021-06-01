// import { memory } from "nbody-physics/physics_bg";
// import * as THREE from 'three';
//
// let camera, scene, renderer, stats, material;
// let mouseX = 0, mouseY = 0;
//
// let windowHalfX = window.innerWidth / 2;
// let windowHalfY = window.innerHeight / 2;
//
// init();
// animate();
//
// let particles = 0;
//
// function init() {
//
//     camera = new THREE.PerspectiveCamera( 55, window.innerWidth / window.innerHeight, 2, 2000 );
//     camera.position.z = 1000;
//
//     scene = new THREE.Scene();
//     scene.fog = new THREE.FogExp2( 0x000000, 0.001 );
//
//     const geometry = new THREE.BufferGeometry();
//     const vertices = [];
//
//     const sprite = new THREE.TextureLoader().load( 'disc.png' );
//
//     for ( let i = 0; i < 10000; i ++ ) {
//
//         const x = 2000 * Math.random() - 1000;
//         const y = 2000 * Math.random() - 1000;
//         const z = 2000 * Math.random() - 1000;
//
//         vertices.push( x, y, z );
//     }
//
//     geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( vertices, 3 ) );
//
//     material = new THREE.PointsMaterial( { size: 35, sizeAttenuation: true, map: sprite, alphaTest: 0.5, transparent: true } );
//     material.color.setHSL( 1.0, 0.3, 0.7 );
//
//     const particles = new THREE.Points( geometry, material );
//     scene.add( particles );
//
//     //
//
//     renderer = new THREE.WebGLRenderer();
//     renderer.setPixelRatio( window.devicePixelRatio );
//     renderer.setSize( window.innerWidth, window.innerHeight );
//     document.body.appendChild( renderer.domElement );
//
//
//     document.body.style.touchAction = 'none';
//     document.body.addEventListener( 'pointermove', onPointerMove );
//
//     window.addEventListener( 'resize', onWindowResize );
//
// }
//
// function onWindowResize() {
//
//     windowHalfX = window.innerWidth / 2;
//     windowHalfY = window.innerHeight / 2;
//
//     camera.aspect = window.innerWidth / window.innerHeight;
//     camera.updateProjectionMatrix();
//
//     renderer.setSize( window.innerWidth, window.innerHeight );
//
// }
//
// function onPointerMove( event ) {
//
//     if ( event.isPrimary === false ) return;
//
//     mouseX = event.clientX - windowHalfX;
//     mouseY = event.clientY - windowHalfY;
//
// }
//
// //
//
// function animate() {
//
//     requestAnimationFrame( animate );
//
//     render();
// }
//
// function render() {
//
//     const time = Date.now() * 0.00005;
//
//     camera.position.x += ( mouseX - camera.position.x ) * 0.05;
//     camera.position.y += ( - mouseY - camera.position.y ) * 0.05;
//     // camera.position.y += 5;
//
//
//
//     camera.lookAt( scene.position );
//
//     const h = ( 360 * ( 1.0 + time ) % 360 ) / 360;
//     material.color.setHSL( h, 0.5, 0.5 );
//
//     renderer.render( scene, camera );
//
// }

import { Simulator } from "nbody-physics";


const NUM_PARTICLES = 50;
const canvas = document.getElementById("nbody-canvas");
let simulator = Simulator.new(NUM_PARTICLES);

const ctx = canvas.getContext("2d");

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const renderLoop = async () => {
    drawBodies();
    await sleep(5000);
    simulator.next_state();
    requestAnimationFrame(renderLoop);
}

const drawBodies = () => {
    // We want to go into the buffer allocated for WASM with an offset of
    ctx.clearRect(0,0, canvas.width, canvas.height);
    for (let i = 0; i < NUM_PARTICLES; i++) {
        console.log(simulator.get_x(i), simulator.get_y(i));
        ctx.beginPath();
        ctx.arc(
            simulator.get_x(i),
            simulator.get_y(i),
            3,
            0,
            2 * Math.PI,
            false
        );
        ctx.fillStyle = 'blue';
        ctx.fill();
    }
}

const main = () => {
    renderLoop();
}

main();



