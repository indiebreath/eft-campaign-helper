<script setup>
import { invoke } from "@tauri-apps/api/core";

invoke('setup');
</script>

<template>
    <div>
        <h1>EFT Campaign Helper</h1>
        <div id="weapons">
            <h2>Weapons</h2>
            <div id="primary">
                <h3>Primary Weapon</h3>
                <p>Name: {{ primary.name }}</p>
            </div>
        </div>
    </div>
</template>
