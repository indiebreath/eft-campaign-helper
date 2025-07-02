<script setup>
import { invoke } from "@tauri-apps/api/core";

async function getGunNames() {
    const res = await invoke("get_gun_names");
    const result = await res;
    return result;
}

const GUN_NAMES = await getGunNames();
console.log(GUN_NAMES);
</script>

<template>
    <div>
        <h1>EFT Campaign Helper</h1>
        <div id="weapons">
            <h2>Weapons</h2>
            <div id="primary">
                <h3>Primary Weapon</h3>
                <p>Name: </p>
                <select name="primary-weapon" id="primary-weapon">
                    <option v-for="x in GUN_NAMES" v-bine:value="x">{{ x }}</option>
                </select>
            </div>
        </div>
    </div>
</template>
