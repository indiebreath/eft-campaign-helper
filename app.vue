<script setup ts="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const GUNS = ref({
    GUN_NAMES: await invoke("get_gun_names"),
    primary: ref(await invoke("get_gun", { gunName: "9A-91" })),
});

const ATTACHMENTS = ref({
    MAGAZINE_NAMES: await invoke("get_magazine_names", {
        gunName: GUNS.value.primary.name,
    }),
    primaryMagazine: await invoke("get_magazine", {
        magazineName: "9A-91 20-Round",
    }),
});

const AMMO = ref({
    AMMO_NAMES: await invoke("get_ammo_names", {
        ammoName: GUNS.value.primary.cartridge,
    }),
    primaryAmmo: await invoke("get_ammo", {
        ammoName: GUNS.value.primary.cartridge,
        roundName: GUNS.value.primary.ammo,
    }),
    primaryAmount: [
        ATTACHMENTS.value.primaryMagazine.capacity,
        ATTACHMENTS.value.primaryMagazine.capacity,
        180,
    ],
    secondary: [0, 0, 0],
    tertiary: [0, 0, 0],
});

function updatePrimaryRecoil() {
    const recoil = AMMO.value.primaryAmmo.recoil;
    GUNS.value.primary.recoil = recoil;
}
updatePrimaryRecoil();

function updatePrimaryAccuracy() {
    const accuracy = AMMO.value.primaryAmmo.accuracy;
    GUNS.value.primary.accuracy = accuracy;
}
updatePrimaryAccuracy();

async function primaryUpdateAttachments() {
    ATTACHMENTS.value.MAGAZINE_NAMES = await invoke("get_magazine_names", {
        gunName: GUNS.value.primary.name,
    });
    ATTACHMENTS.value.primaryMagazine = await invoke("get_magazine", {
        magazineName: MAGAZINE_NAMES[0],
    });
}

async function primaryUpdateAmmo() {
    AMMO.value.AMMO_NAMES = await invoke("get_ammo_names", {
        ammoName: GUNS.value.primary.cartridge,
    });
    AMMO.value.primaryAmmo = await invoke("get_ammo", {
        ammoName: GUNS.value.primary.cartridge,
        roundName: GUNS.value.primary.ammo,
    });
    AMMO.value.primaryAmount[0] = ATTACHMENTS.value.primaryMagazine.capacity;
    AMMO.value.primaryAmount[1] = ATTACHMENTS.value.primaryMagazine.capacity;
    updatePrimaryRecoil();
    updatePrimaryAccuracy();
}

async function primaryChange(event) {
    GUNS.value.primary = await invoke("get_gun", {
        gunName: event.target.value,
    });
    primaryUpdateAttachments();
    primaryUpdateAmmo();
}

async function primaryChangeMagazine(event) {
    ATTACHMENTS.value.primaryMagazine = await invoke("get_magazine", {
        magazineName: event.target.value,
    });
    primaryUpdateAmmo();
}

async function primaryChangeAmmo(event) {
    AMMO.value.primaryAmmo = await invoke("get_ammo", {
        ammoName: GUNS.value.primary.cartridge,
        roundName: event.target.value,
    });
    updatePrimaryRecoil();
    updatePrimaryAccuracy();
}

function primaryChangeAmmoTotal(event) {
    AMMO.value.primaryAmount[2] = event.target.value;
}

function primaryFire() {
    if (AMMO.value.primaryAmount[0] > 0) {
        AMMO.value.primaryAmount[0]--;
    }
}

function primaryReload() {
    const diff = AMMO.value.primaryAmount[1] - AMMO.value.primaryAmount[0];
    if (diff > AMMO.value.primaryAmount[2]) {
        AMMO.value.primaryAmount[0] =
            parseInt(AMMO.value.primaryAmount[2]) +
            parseInt(AMMO.value.primaryAmount[0]);
        AMMO.value.primaryAmount[2] = 0;
        return;
    }

    AMMO.value.primaryAmount[0] = ATTACHMENTS.value.primaryMagazine.capacity;
    AMMO.value.primaryAmount[2] -= diff;
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
                        @change="primaryChange($event)"
                    >
                        <option v-for="a in GUNS.GUN_NAMES" :key="a" :value="a">
                            {{ a }}
                        </option>
                    </select>

                    <p>Cartridge: {{ GUNS.primary.cartridge }}</p>

                    <p>Range: {{ GUNS.primary.range }}m</p>

                    <p>Firing Modes:</p>
                    <p v-if="GUNS.primary.semi">Semi-Auto</p>
                    <p v-if="GUNS.primary.full != 0">
                        Full-Auto ({{ GUNS.primary.full }})
                    </p>
                    <p v-if="GUNS.primary.burst[0] != 0">
                        Burst ({{ GUNS.primary.burst[0] }},
                        {{ GUNS.primary.burst[1] }})
                    </p>

                    <p>Attachments:</p>
                    <label for="primaryMagazine">Magazine:</label>
                    <select
                        v-if="GUNS.primary.attachments.includes('Magazine')"
                        id="primaryMagazine"
                        name="primaryMagazine"
                        @change="primaryChangeMagazine($event)"
                    >
                        <option
                            v-for="i in ATTACHMENTS.MAGAZINE_NAMES"
                            :key="i"
                            :value="i"
                        >
                            {{ i }}
                        </option>
                    </select>

                    <p>Weight: {{ GUNS.primary.weight }}kg</p>
                    <p>Size: {{ GUNS.primary.size }}</p>
                </div>

                <div id="primary-ammo">
                    <p>Ammo:</p>
                    <select
                        id="primaryAmmo"
                        name="primaryAmmo"
                        @change="primaryChangeAmmo($event)"
                    >
                        <option
                            v-for="z in AMMO.AMMO_NAMES"
                            :key="z"
                            :value="z"
                        >
                            {{ z }}
                        </option>
                    </select>

                    <p>Damage: {{ AMMO.primaryAmmo.damage }}</p>
                    <p>Penetration: {{ AMMO.primaryAmmo.penetration }}</p>

                    <p>Accuracy: {{ GUNS.primary.accuracy }}</p>
                    <p>Recoil: {{ GUNS.primary.recoil }}</p>
                </div>

                <div id="primary-function">
                    <p>Current Ammo: {{ AMMO.primaryAmount[0] }}</p>
                    <p>Max Ammo: {{ AMMO.primaryAmount[1] }}</p>

                    <label for="totalAmmo">Total Ammo:</label>
                    <input
                        id="totalPrimaryAmmo"
                        type="text"
                        name="totalPrimaryAmmo"
                        :value="AMMO.primaryAmount[2]"
                        @change="primaryChangeAmmoTotal($event)"
                    ><br >

                    <button id="firePrimary" @click="primaryFire()">
                        Fire
                    </button>
                    <button id="primaryReload" @click="primaryReload()">
                        Reload
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
