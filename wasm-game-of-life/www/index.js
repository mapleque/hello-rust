import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext("2d");

class Game {
    constructor() {
        this.is_pause = false;
        this.universe = null;
        this.width = 0;
        this.height = 32;
    }

    render() {
        document.getElementById("generation").value =
            this.universe.current_generation();

        this.drawGrid();
        this.drawCells();
    }

    renderLoop() {
        if (this.is_pause) {
            return;
        }
        this.universe.tick();
        this.render();
        requestAnimationFrame(() => this.renderLoop());
    }

    drawGrid() {
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;

        for (let i = 0; i <= this.width; i++) {
            ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
            ctx.lineTo(
                i * (CELL_SIZE + 1) + 1,
                (CELL_SIZE + 1) * this.height + 1
            );
        }

        for (let j = 0; j <= this.height; j++) {
            ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
            ctx.lineTo(
                (CELL_SIZE + 1) * this.width + 1,
                j * (CELL_SIZE + 1) + 1
            );
        }

        ctx.stroke();
    }

    getIndex(row, column) {
        return row * this.width + column;
    }

    drawCells() {
        const cellsPtr = this.universe.cells();
        const cells = new Uint8Array(
            memory.buffer,
            cellsPtr,
            this.width * this.height
        );

        ctx.beginPath();

        for (let row = 0; row < this.height; row++) {
            for (let col = 0; col < this.width; col++) {
                const idx = this.getIndex(row, col);

                ctx.fillStyle =
                    cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

                ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        ctx.stroke();
    }

    init() {
        if (this.universe != null) {
            return false;
        }
        let width = parseInt(document.getElementById("width").value);
        let height = parseInt(document.getElementById("height").value);
        let code = document.getElementById("code").value;
        this.universe = Universe.new(width, height);
        this.width = width;
        this.height = height;
        let universe = this.universe;
        eval(code);
        canvas.height = (CELL_SIZE + 1) * height + 1;
        canvas.width = (CELL_SIZE + 1) * width + 1;
        this.render();
        return true;
    }

    start() {
        this.init();
        this.is_pause = false;
        this.renderLoop();
    }

    step() {
        if (this.init()) {
            return;
        }
        this.universe.tick();
        this.render();
    }

    pause() {
        this.is_pause = true;
    }

    reset() {
        this.is_pause = true;
        this.universe = null;
        game.init();
    }
}

const game = new Game();

document.getElementById("start").onclick = () => game.start();
document.getElementById("step").onclick = () => game.step();
document.getElementById("pause").onclick = () => game.pause();
document.getElementById("reset").onclick = () => game.reset();

game.init();
