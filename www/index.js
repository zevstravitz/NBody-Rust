// import { memory } from "nbody-physics/physics_bg";
import * as THREE from 'three';
import {Simulator} from "nbody-physics";

const NUM_PARTICLES = 1000;
const DIMENSIONS = 3;
let simulator = Simulator.new(NUM_PARTICLES);

let camera, scene, renderer, material, particles;
let mouseX = 0, mouseY = 0;

let windowHalfX = window.innerWidth / 2;
let windowHalfY = window.innerHeight / 2;

init();
animate();

function init() {

    camera = new THREE.PerspectiveCamera( 45, window.innerWidth / window.innerHeight, 2, 10000 );
    camera.position.z = 4000;

    scene = new THREE.Scene();
    // scene.fog = new THREE.FogExp2( 0x000000, 0.0001 );

    const geometry = new THREE.BufferGeometry();
    const vertices = [];

    const sprite = new THREE.TextureLoader().load( 'ball.png' );

    for ( let i = 0; i < NUM_PARTICLES; i ++ ) {

        const x = simulator.get_dim(i, 0);
        const y = simulator.get_dim(i, 1);
        const z = simulator.get_dim(i, 2);

        vertices.push( x, y, z );
    }

    // for ( let i = 0; i < NUM_PARTICLES; i ++ ) {
    //     console.log(simulator.get_dim(i, 0), simulator.get_dim(i, 1), simulator.get_dim(i, 2));
    // }

    geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( vertices, DIMENSIONS ) );

    material = new THREE.PointsMaterial( { size: 55, sizeAttenuation: true, map: sprite, alphaTest: 0.5, transparent: true } );
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
    // document.body.addEventListener( 'scroll', onScroll );

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


// let lastScrollTop = 0;
// function onScroll ( event ) {
//     if ( event.isPrimary === false ) return;
//
//     let st = window.pageYOffset || document.documentElement.scrollTop;
//     if (st > lastScrollTop){
//         camera.position.z -= 10;
//     } else {
//         camera.position.z += 10;
//     }
//     lastScrollTop = st <= 0 ? 0 : st;
// }


function animate() {

    requestAnimationFrame( animate );

    render();
}

async function render() {

    const time = Date.now() * 0.00005;
    camera.position.x = 1000 + windowHalfX - mouseX*2;
    camera.position.y = 500 + windowHalfY + mouseY*2;

    camera.lookAt( scene.position );

    const positions = particles.geometry.attributes.position.array;
    // console.log(positions);

    for (let i = 0; i < NUM_PARTICLES; i++) {
        for (let dim = 0; dim < DIMENSIONS; dim++) {
            positions[i*DIMENSIONS+dim] = simulator.get_dim(i, dim);
        }
    }

    // WHERE WE UPDATE THE POSITIONS
    simulator.next_state();

    // await sleep(5000);
    // console.log(simulator.get_dim(0, 0));

    particles.geometry.attributes.position.needsUpdate = true;

    const h = ( 360 * ( 1.0 + time ) % 360 ) / 360;
    material.color.setHSL( h, 0.5, 0.5 );

    renderer.render( scene, camera );

}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}


