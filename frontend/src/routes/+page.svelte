<script lang="ts">
    let pallet1: HTMLElement;
    let pallet2: HTMLElement;
    let ball: HTMLElement;

    async function handleKeyDown(event: KeyboardEvent) {
        if (event.key === "w") {
            movePalletDelta(pallet1, -20);
        } else if (event.key === "s") {
            movePalletDelta(pallet1, 20);
        } else if (event.key === "ArrowUp") {
            movePalletDelta(pallet2, -20);
        } else if (event.key === "ArrowDown") {
            movePalletDelta(pallet2, 20);
        } else if (event.key === " ") {
            await demo();
        }
    }

    async function demo() {
        moveBall(0, 0);
        await sleep(1000);
        moveBall(100, 100);
        await sleep(1000);
        moveBall(200, 200);
        await sleep(1000);
        moveBall(300, 300);
        await sleep(1000);
        moveBall(400, 400);
    }

    function movePalletDelta(element: HTMLElement, delta: number) {
        let newTop = clamp(
            element.offsetTop + delta,
            element.clientHeight / 2,
            window.innerHeight - element.clientHeight / 2
        );
        element.style.top = `${newTop}px`;
    }

    function movePallet(element: HTMLElement, y: number) {
        let newTop = clamp(
            y,
            element.clientHeight / 2,
            window.innerHeight - element.clientHeight / 2
        );
        element.style.top = `${newTop}px`;
    }

    function moveBall(x: number = 0, y: number = 0) {
        let newTop = clamp(
            x,
            ball.clientHeight / 2,
            window.innerHeight - ball.clientHeight / 2
        );
        let newLeft = clamp(
            y,
            ball.clientWidth / 2,
            window.innerWidth - ball.clientWidth / 2
        );
        ball.style.top = `${newTop}px`;
        ball.style.left = `${newLeft}px`;
    }

    const clamp = (num: number, min: number, max: number) =>
        Math.min(Math.max(num, min), max);

    const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));
</script>

<svelte:window on:keydown={handleKeyDown} />

<div class="main">
    <div class="ball" bind:this={ball} />

    <div class="pallet1" bind:this={pallet1} />
    <div class="pallet2" bind:this={pallet2} />
</div>

<style>
    .main {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: #000;
    }

    .ball {
        position: absolute;
        top: 50%;
        left: 50%;
        width: 20px;
        height: 20px;
        background: #fff;
        border-radius: 50%;
        transform: translate(-50%, -50%);
    }

    .pallet1 {
        position: absolute;
        top: 50%;
        left: 20px;
        width: 20px;
        height: 200px;
        background: #fff;
        transform: translate(0, -50%);
    }

    .pallet2 {
        position: absolute;
        top: 50%;
        right: 20px;
        width: 20px;
        height: 200px;
        background: #fff;
        transform: translate(0, -50%);
    }
</style>
