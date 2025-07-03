<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const GUN_NAMES = await invoke("get_gun_names");

const primary = ref(await invoke("get_gun", { gunName: "ADAR 2-15" }));
const ammo = ref({
    primary: await invoke("get_ammo", { ammoName: primary.value.cartridge, roundName: primary.value.ammo }),
    primaryAmount: [primary.value.max, primary.value.max, 180],
    secondary: [0, 0, 0],
    tertiary: [0, 0, 0]
})

async function changePrimary(event) {
    primary.value = await invoke("get_gun", { gunName: event.target.value });
    ammo.value.primaryAmount[0] = primary.value.max;
    ammo.value.primaryAmount[1] = primary.value.max;
}

function changePrimaryAttachment(event) {
    let position = primary.value.attachments.indexOf(event.target.id);
    primary.value.attachments[position] = event.target.value;
}

function changePrimaryMax(event) {
    ammo.value.primaryAmount[1] = event.target.value;
    console.log(ammo.value.primaryAmount[1]);
}

function firePrimary() {
    ammo.value.primaryAmount[0]--;
}

function reloadPrimary() {
    const diff = ammo.value.primaryAmount[1] - ammo.value.primary[0];
    ammo.value.primaryAmount[0] = ammo.value.primary[1];
    ammo.value.primaryAmount[2] -= diff;
}

function changePrimaryTotal(event) {
    ammo.value.primaryAmount[2] = event.target.value;
}
</script>

<template>
    <div>
        <h1>EFT Campaign Helper</h1>

        <div id="weapons">
            <h2>Weapons</h2>

            <div id="primary">
                <h3>Primary Weapon</h3>

                <div id="primary-info">
                    <p>Name:</p>
                    <select
                        @change="changePrimary($event)"
                        name="primary-weapon"
                        id="primary-weapon"
                    >
                        <option v-for="x in GUN_NAMES" v-bind:value="x">
                            {{ x }}
                        </option>
                    </select>

                    <p>Cartridge: {{ primary.cartridge }}</p>
                    <p>Ammo: {{ primary.ammo }}</p>
                    <p>Range: {{ primary.range }}m</p>

                    <p>Firing Modes:</p>
                    <p v-if="primary.semi">Semi-Auto</p>
                    <p v-if="primary.full != 0">
                        Full-Auto ({{ primary.full }})
                    </p>
                    <p v-if="primary.burst[0] != 0">
                        Burst ({{ primary.burst[0] }}, {{ primary.burst[1] }})
                    </p>


                    <p>Accuracy: {{ primary.accuracy }}</p>
                    <p>Recoil: {{ primary.recoil }}</p>
                    <p>Weight: {{ primary.weight }}kg</p>
                    <p>Size: {{ primary.size }}</p>
                </div>

                <div id="primary-function">
                    <div v-if="primary.attachments[0]">
                        <label for="primaryAttachments">Attachments:</label>
                        <input type="text" class="primaryAttachments" name="primaryAttachments" v-for="y in primary.attachments" @change="changePrimaryAttachment($event)" v-bind:value="y" v-bind:id="y"></input>
                    </div>

                    <p>Current Ammo: {{ ammo.primary[0] }}</p>
                    <button id="firePrimary" v-on:click="firePrimary()">Fire</button>

                    <label for="maxPrimaryAmmo">Max Ammo:</label>
                    <input type="text" id="maxPrimaryAmmo" name="maxPrimaryAmmo" @change="changePrimaryMax($event)" v-bind:value="ammo.primary[1]"></input>
                    <button id="primaryReload" v-on:click="reloadPrimary()">Reload</button>

                    <label for="totalAmmo">Total Ammo:</label> <input type="text" id="totalPrimaryAmmo" name="totalPrimaryAmmo" @change="changePrimaryTotal($event)" v-bind:value="ammo.primary[2]"></input>
                </div>
            </div>
        </div>
    </div>
</template>
