<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

async function getGunNames() {
    const res = await invoke("get_gun_names");
    const result = await res;
    return result;
}

async function changeGun(event) {
    primary.value = await invoke("get_gun", { gunName: event.target.value });
    console.log(primary);
}

const GUN_NAMES = await getGunNames();
const primary = ref(await invoke("get_gun", { gunName: "9A-91" }));

</script>

<template>
    <div>
        <h1>EFT Campaign Helper</h1>
        <div id="weapons">
            <h2>Weapons</h2>
            <div id="primary">
                <h3>Primary Weapon</h3>

                <p>Name:</p>
                <select
                    v-on:submit="changeGun"
                    @change="changeGun($event)"
                    name="primary-weapon"
                    id="primary-weapon"
                >
                    <option v-for="x in GUN_NAMES" v-bind:value="x">
                        {{ x }}
                    </option>
                </select>

                <p>Ammo: {{ primary.ammo }}</p>
            </div>
        </div>
    </div>
</template>
