<script lang="ts">
    import { onMount } from "svelte";

    let socket: WebSocket;
    let current_id: string;

    let pallet1: HTMLElement;
    let pallet2: HTMLElement;
    let ball: HTMLElement;

    onMount(() => {
        socket = new WebSocket("ws://localhost:8080");
        socket.onmessage = (event) => {
            processSocketMsg(event);
        };

        socket.onopen = () => {
            console.log("Connected to socket");
            getState();
        };

        socket.onclose = () => {
            console.log("Disconnected from socket");
        };
    });

    function getState() {
        if (socket.readyState != WebSocket.OPEN) {
            return;
        }

        socket.send("get_state");
        setTimeout(getState, 16);
    }

    function processSocketMsg(event: MessageEvent) {
        let parts = event.data.split(",");
        if (parts[0] == "id") {
            current_id = parts[1];
        } else if (parts[0] == "pallet_pos") {
            let pallet = parts[1];
            let y = parseInt(parts[2]);
            if (pallet == "1") {
                movePallet(pallet1, y);
            } else if (pallet == "2") {
                movePallet(pallet2, y);
            }
        } else if (parts[0] == "ball_pos") {
            let x = parseInt(parts[1]);
            let y = parseInt(parts[2]);

            let x_ratio = window.innerWidth / 100;
            let y_ratio = window.innerHeight / 50;

            x = x * x_ratio;
            y = y * y_ratio;

            moveBall(x, y);
        } else if (parts[0] == "bot_pos") {
            let y = parseInt(parts[1]);
            let y_ratio = window.innerHeight / 50;
            y = y * y_ratio;
            movePallet(pallet2, y);
        }
    }

    async function handleKeyDown(event: KeyboardEvent) {
        if (event.key === "w") {
            moveClientDelta(-20);
        } else if (event.key === "s") {
            moveClientDelta(20);
        }
    }

    function moveClientDelta(delta: number) {
        if (current_id == "1") {
            movePalletDelta(pallet1, delta);
            let y_ratio = 50 / window.innerHeight;
            let y = pallet1.offsetTop * y_ratio;

            socket.send(`pallet_pos,${y}`);
        } else if (current_id == "2") {
            movePalletDelta(pallet2, delta);

            let y_ratio = 50 / window.innerHeight;
            let y = pallet2.offsetTop * y_ratio;

            socket.send(`pallet_pos,${y}`);
        }
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
            y,
            ball.clientHeight / 2,
            window.innerHeight - ball.clientHeight / 2
        );
        let newLeft = clamp(
            x,
            ball.clientWidth / 2,
            window.innerWidth - ball.clientWidth / 2
        );
        ball.style.top = `${newTop}px`;
        ball.style.left = `${newLeft}px`;
    }

    const clamp = (num: number, min: number, max: number) =>
        Math.min(Math.max(num, min), max);
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
