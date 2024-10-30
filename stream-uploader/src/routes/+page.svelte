<script lang="ts">
    import ObsStatus from "../components/ObsStatus.svelte";
</script>

<main class="container mx-auto px-4 py-8">
    <h1 class="text-3xl font-bold text-center mb-8">Stream Uploader</h1>

    <ObsStatus />

    <div class="mt-8 bg-white rounded-lg shadow p-6">
        <h2 class="text-xl font-semibold mb-4">Settings</h2>
        <!-- Add settings form here -->
    </div>

    <div class="mt-8 bg-white rounded-lg shadow p-6">
        <h2 class="text-xl font-semibold mb-4">Recent Uploads</h2>
        <!-- Add upload history here -->
    </div>
</main>
