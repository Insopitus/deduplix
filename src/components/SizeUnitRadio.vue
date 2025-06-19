<script setup lang="ts">
import { SizeUnit } from "../types";
defineProps<{
    options: Array<SizeUnit>;
    modelValue: SizeUnit;
}>();
const emit = defineEmits(["update:modelValue", "change", "blur"]);

function updateValue(option: SizeUnit) {
    emit("update:modelValue", option);
}
</script>
<template>
    <div class="size-unit-radio">
        <button
            @click="updateValue(option)"
            :class="{ selected: modelValue === option }"
            v-for="option in options"
        >
            {{ option }}
        </button>
    </div>
</template>

<style scoped>
.size-unit-radio{
    flex-grow: 1;
}
button {
    width: 28px;
    height: 20px;
    padding: 2px 3px;
    font-size: 14px;
    box-shadow: 0 0 1px black;
}
.selected {
    background: aquamarine;
    /* color: white; */
}
</style>
