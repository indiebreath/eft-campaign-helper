<script setup ts="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const GUN_NAMES = await invoke("get_gun_names");

const primary = ref(await invoke("get_gun", { gunName: "9A-91" }));
const ammo = ref({
    primary: await invoke("get_ammo", {
        ammoName: primary.value.cartridge,
        roundName: primary.value.ammo,
    }),
    primaryAmount: [primary.value.max, primary.value.max, 180],
    secondary: [0, 0, 0],
    tertiary: [0, 0, 0],
});

const AMMO_NAMES = ref(
    await invoke("get_ammo_names", { ammoName: primary.value.cartridge }),
);

function updatePrimaryRecoil() {
    const recoil = ammo.value.primary.recoil;
    primary.value.recoil = recoil;
}
updatePrimaryRecoil();

function updatePrimaryAccuracy() {
    const accuracy = ammo.value.primary.accuracy;
    primary.value.accuracy = accuracy;
}
updatePrimaryAccuracy();

async function changePrimary(event) {
    primary.value = await invoke("get_gun", { gunName: event.target.value });
    ammo.value.primaryAmount[0] = primary.value.max;
    ammo.value.primaryAmount[1] = primary.value.max;
}

async function changePrimaryAmmo(event) {
    ammo.value.primary = await invoke("get_ammo", {
        ammoName: primary.value.cartridge,
        roundName: event.target.value,
    });
    AMMO_NAMES.value = await invoke("get_ammo_names", {
        ammoName: primary.value.cartridge,
    });
    updatePrimaryRecoil();
    updatePrimaryAccuracy();
}

function changePrimaryAttachment(event) {
    const position = primary.value.attachments.indexOf(event.target.id);
    primary.value.attachments[position] = event.target.value;
}

function changePrimaryMax(event) {
    ammo.value.primaryAmount[1] = event.target.value;
}

function firePrimary() {
    ammo.value.primaryAmount[0]--;
    if (ammo.value.primaryAmount[0] < 0) {
        ammo.value.primaryAmount[0] = 0;
    }
}

function reloadPrimary() {
    const diff = ammo.value.primaryAmount[1] - ammo.value.primaryAmount[0];
    ammo.value.primaryAmount[0] = ammo.value.primaryAmount[1];
    ammo.value.primaryAmount[2] = ammo.value.primaryAmount[2] - diff;
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
                        id="primaryWeapon"
                        name="primaryWeapon"
                        @change="changePrimary($event)"
                    >
                        <option v-for="x in GUN_NAMES" :key="x" :value="x">
                            {{ x }}
                        </option>
                    </select>

                    <p>Cartridge: {{ primary.cartridge }}</p>

                    <p>Range: {{ primary.range }}m</p>

                    <p>Firing Modes:</p>
                    <p v-if="primary.semi">Semi-Auto</p>
                    <p v-if="primary.full != 0">
                        Full-Auto ({{ primary.full }})
                    </p>
                    <p v-if="primary.burst[0] != 0">
                        Burst ({{ primary.burst[0] }}, {{ primary.burst[1] }})
                    </p>

                    <div v-if="primary.attachments[0]">
                        <label for="primaryAttachments">Attachments:</label>
                        <input
                            v-for="y in primary.attachments"
                            :id="y"
                            :key="y"
                            type="text"
                            class="primaryAttachments"
                            name="primaryAttachments"
                            :value="y"
                            @change="changePrimaryAttachment($event)"
                        >
                    </div>

                    <p>Weight: {{ primary.weight }}kg</p>
                    <p>Size: {{ primary.size }}</p>
                </div>

                <div id="primary-ammo">
                    <p>Ammo:</p>
                    <select
                        id="primaryAmmo"
                        name="primaryAmmo"
                        @change="changePrimaryAmmo($event)"
                    >
                        <option v-for="z in AMMO_NAMES" :key="z" :value="z">
                            {{ z }}
                        </option>
                    </select>

                    <p>Damage: {{ ammo.primary.damage }}</p>
                    <p>Penetration: {{ ammo.primary.penetration }}</p>

                    <p>Accuracy: {{ primary.accuracy }}</p>
                    <p>Recoil: {{ primary.recoil }}</p>
                </div>

                <div id="primary-function">
                    <p>Current Ammo: {{ ammo.primaryAmount[0] }}</p>

                    <label for="maxPrimaryAmmo">Max Ammo:</label>
                    <input
                        id="maxPrimaryAmmo"
                        type="text"
                        name="maxPrimaryAmmo"
                        :value="ammo.primaryAmount[1]"
                        @change="changePrimaryMax($event)"
                    ><br >

                    <label for="totalAmmo">Total Ammo:</label>
                    <input
                        id="totalPrimaryAmmo"
                        type="text"
                        name="totalPrimaryAmmo"
                        :value="ammo.primaryAmount[2]"
                        @change="changePrimaryTotal($event)"
                    ><br >

                    <button id="firePrimary" @click="firePrimary()">
                        Fire
                    </button>
                    <button id="primaryReload" @click="reloadPrimary()">
                        Reload
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
