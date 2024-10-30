<script lang="ts">
    import { recordingState, connectionState } from "$lib/stores/obs";

    let isConnected = $derived($connectionState.connected);
    let lastConnected = $derived($connectionState.lastConnected);
    let lastDisconnected = $derived($connectionState.lastDisconnected);

    let isRecording = $derived($recordingState.isRecording);
    let currentFile = $derived($recordingState.currentFile);
    let lastRecording = $derived($recordingState.lastRecording);

    // Format time values
    let lastConnectedText = $derived(
        lastConnected ? new Date(lastConnected).toLocaleString() : null,
    );

    let lastDisconnectedText = $derived(
        lastDisconnected ? new Date(lastDisconnected).toLocaleString() : null,
    );
</script>

<div class="bg-gray-100 p-4 rounded-lg">
    <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-2">
            <div
                class={`w-3 h-3 rounded-full ${isConnected ? "bg-green-500" : "bg-red-500"}`}
            ></div>
            <span class="font-medium"
                >OBS {isConnected ? "Connected" : "Disconnected"}</span
            >
        </div>

        {#if isRecording}
            <div class="flex items-center gap-2">
                <div
                    class="w-3 h-3 rounded-full bg-red-500 animate-pulse"
                ></div>
                <span class="text-red-500 font-medium">Recording</span>
            </div>
        {/if}
    </div>

    <div class="space-y-2 text-sm text-gray-600">
        {#if currentFile}
            <div>
                <span class="font-medium">Current Recording:</span>
                {currentFile}
            </div>
        {/if}

        {#if lastRecording}
            <div>
                <span class="font-medium">Last Recording:</span>
                {lastRecording}
            </div>
        {/if}

        {#if lastConnectedText && isConnected}
            <div>
                <span class="font-medium">Connected:</span>
                {lastConnectedText}
            </div>
        {/if}

        {#if lastDisconnectedText && !isConnected}
            <div>
                <span class="font-medium">Last Seen:</span>
                {lastDisconnectedText}
            </div>
        {/if}
    </div>
</div>
