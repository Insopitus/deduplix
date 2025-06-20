<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { Ref, ref } from "vue";
import LabeledInput from "./LabeledInput.vue";
import SizeUnitRadio from "./SizeUnitRadio.vue";
const emit = defineEmits(["path-selected"]);
import { SizeUnit } from "../types";
const KILO_BYTE = 1024;
const MEGA_BYTE = KILO_BYTE * 1024;
const GIGA_BYTE = MEGA_BYTE * 1024;
const include = ref("");
const exclude = ref("");
const minSize = ref(1);
const minSizeUnit: Ref<SizeUnit> = ref("KB");
const maxSize = ref(1);
const maxSizeUnit: Ref<SizeUnit> = ref("GB");

const MIN_SIZE_VALUE = 0;
const MAX_SIZE_VALUE = 999999;

async function chooseDirectory() {
    open({
        multiple: false,
        directory: true,
    })
        .then((path) => {
            console.log(path);
            if (path) {
                console.log(typeof path);
                emit("path-selected", {
                    path,
                    config: {
                        include: include.value,
                        exclude: exclude.value,
                        size_extend: [
                            convertSize(minSize.value, minSizeUnit.value),
                            convertSize(maxSize.value, maxSizeUnit.value),
                        ],
                    },
                });
            }
        })
        .catch((e) => {
            console.log(e);
        });
}
function validateSize() {
    if (minSize.value < MIN_SIZE_VALUE) {
        minSize.value = MIN_SIZE_VALUE;
    }
    if (maxSize.value > MAX_SIZE_VALUE) {
        maxSize.value = MAX_SIZE_VALUE;
    }
}
function convertSize(num: number, unit: SizeUnit) {
    let result = num;
    switch (unit) {
        case "B":
            return result;
        case "KB":
            return result * KILO_BYTE;
        case "MB":
            return result * MEGA_BYTE;
        case "GB":
            return result * GIGA_BYTE;
        default:
            return 0;
    }
}
</script>

<template>
    <div class="home">
        <div class="mask"></div>
        <div class="main">
            <button @click="chooseDirectory">Choose a directory...</button>
            <LabeledInput
                v-model="include"
                label="include:"
                style="width: 100%"
                placeholder="e.g. **/*.txt, **/*.mp4 Separate by commas"
            />
            <LabeledInput
                v-model="exclude"
                label="exclude:"
                placeholder="e.g. **/*.txt, **/*.mp4 Separate by commas"
            />

            <div class="size-line">
                <div class="min-line">
                    <LabeledInput
                        v-model="minSize"
                        label="min size:"
                        type="number"
                        @change="validateSize"
                        placeholder="e.g. 1024"
                    />
                    <SizeUnitRadio
                        v-model="minSizeUnit"
                        name="min-size"
                        label="size unit:"
                        style="position: absolute; right: 4px; top: 10px"
                        :options="['B', 'KB', 'MB', 'GB']"
                    />
                </div>
                <div class="max-line">
                    <LabeledInput
                        v-model="maxSize"
                        label="max size:"
                        type="number"
                        @change="validateSize"
                        placeholder="e.g. 1024"
                    />

                    <SizeUnitRadio
                        v-model="maxSizeUnit"
                        name="max-size"
                        label="size unit:"
                        style="position: absolute; right: 4px; top: 10px"
                        :options="['B', 'KB', 'MB', 'GB']"
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="css" scoped>
.home {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    flex-grow: 1;
}

.main {
    width: 60%;
    min-width: 400px;
    max-width: 800px;
}

button {
    font-size: 24px;
    width: 100%;
    z-index: 1;
    height: 80px;
}

.max-line,
.min-line {
    display: flex;
    position: relative;
}

.mask {
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 0;
}

.size-line {
    display: flex;
    gap: 10px;
}
</style>
