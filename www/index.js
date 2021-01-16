import { Universe, Cell } from "wasmlife";
import { memory } from "wasmlife/wasmlife_bg"

const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const LIVE_COLOR = "#000000";

const CELL_SIZE = 10; //px
const UWIDTH  = 20;
const UHEIGHT = 20;
const INITP = 0.3;

const canvas = document.getElementById("wasmlife-canvas");
canvas.width  = (CELL_SIZE + 1) * UWIDTH + 1;
canvas.height = (CELL_SIZE + 1) * UHEIGHT + 1;
const ctx = canvas.getContext('2d');

let tickms  = 100;
let resetms = 3 * 1000;

let animationId = null;
let start;
let total;
const renderLoop = (timestamp) => {
  if (start === undefined) {
    start = timestamp;
    total = 0; 
  }
  let elapsed = timestamp - start;
  if (elapsed > tickms) {
    //debugger;
    universe.tick();
    start = timestamp;
    total += elapsed;
    elapsed = 0;
  }
  drawGrid();
  drawCells();
  animationId = requestAnimationFrame(renderLoop);
};

const classicButton = document.getElementById("classic");
const speedButton   = document.getElementById("speed");
const pauseButton   = document.getElementById("pause");
const stepButton    = document.getElementById("step");
const resetButton   = document.getElementById("reset");

const reset = () => {
  if (total > resetms) {
    switch (getRandomInt(0,4)) {
      case 0:
        console.log("gliders");
        universe.init_gliders(getRandomInt(0,4));
        tickms  = 500;
        resetms = 5 * 1000;
        break;
      case 1:
        console.log("random");
        universe.init_random(INITP);
        tickms  = 0;
        resetms = 10 * 1000;
        break;
      default:
        console.log("modulo");
        universe.init_modulo();
        tickms  = 80;
        resetms = 5 * 1000;
        break;
    }
    start = timestamp;
    total = 0;
  }
}

const isPaused = () => {
  return animationId === null;
};
const play = () => {
  pauseButton.textContent = "⏸";
  renderLoop();
};
const pause = () => {
  pauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

resetButton.addEventListener("click", event => {
    reset();
});

pauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width  / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;
  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop  = (event.clientY - boundingRect.top)  * scaleY;
  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE+1)), UHEIGHT-1);
  const col = Math.min(Math.floor(canvasLeft/ (CELL_SIZE+1)),  UWIDTH-1);
  universe.toggle_cell(row, col);
  drawGrid();
  drawCells();
});

const getRandomInt = (min,max) => {
  return min+ Math.floor(max * Math.random());
};

const getIndex = (row, col) => {
  return row*UWIDTH + col;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, UWIDTH * UHEIGHT);
  ctx.beginPath();
  for (let row = 0; row < UHEIGHT; row++) {
    for (let col = 0; col < UWIDTH; col++) {
      const idx = getIndex(row, col);
      ctx.fillStyle = cells[idx] == Cell.Dead ? DEAD_COLOR : LIVE_COLOR;
      ctx.fillRect(col*(CELL_SIZE+1)+1, row*(CELL_SIZE+1)+1, CELL_SIZE, CELL_SIZE);
    }
  }
  ctx.stroke();
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;
  for (let i = 0; i <= UWIDTH; i++) { //vertical lines
    ctx.moveTo(i * (CELL_SIZE + 1) + 1    , 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1    , (CELL_SIZE + 1) * UHEIGHT + 1);
  }
  for (let j = 0; j <= UHEIGHT; j++) { //horizontal lines
    ctx.moveTo(0                          , j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * UWIDTH + 1, j * (CELL_SIZE + 1) + 1);
  }
  ctx.stroke();
};

let universe = Universe.new(UWIDTH, UHEIGHT);
//universe.init_modulo(UWIDTH, UHEIGHT);
//universe.init_random(UWIDTH, UHEIGHT, INITP);
universe.init_gliders(UWIDTH, UHEIGHT, 1);
drawGrid();
drawCells();
play();
