<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { Ref, ref } from "vue";
import LabeledInput from "./LabeledInput.vue";
import SizeUnitRadio from "./SizeUnitRadio.vue";
const emit = defineEmits(["path-selected"]);
import { SizeUnit } from "../types";
const include = ref("");
const exclude = ref("");
const minSize = ref(0);
const minSizeUnit: Ref<SizeUnit> = ref("B");
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
                emit("path-selected", {
                    path,
                    filter: [include.value, exclude.value],
                    sizeExtend: [minSize.value, maxSize.value],
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
</script>

<template>
    <div class="home">
        <div class="mask"></div>
        <button @click="chooseDirectory">Choose a directory...</button>
        <LabeledInput
            v-model="include"
            label="include:"
            placeholder="e.g. *.txt, photo.*. Separate by commas"
        />
        <LabeledInput
            v-model="exclude"
            label="exclude:"
            placeholder="e.g. *.txt, downloads/**/*.mp4. Separate by commas"
        />

        <div class="size-line">
            <LabeledInput
                v-model="minSize"
                label="min size:"
                type="number"
                @change="validateSize"
                placeholder="e.g. 1024"
            />
            <LabeledInput
                v-model="maxSize"
                label="max size:"
                type="number"
                @change="validateSize"
                placeholder="e.g. 1024"
            />
        </div>
        <SizeUnitRadio
            v-model="minSizeUnit"
            name="min-size"
            label="size unit:"
            :options="['B', 'KB', 'MB', 'GB']"
        />
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

button {
    font-size: 24px;
    width: 60%;
    z-index: 1;
    height: 80px;
}
input {
    outline: none;
    margin-bottom: 10px;
    width: 60%;
    resize: none;
    position: relative;
}
input > label {
    position: absolute;
    top: -4px;
    left: 4px;
}
input[type="number"] {
    width: 45%;
}

.mask {
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 0;
}
</style>
