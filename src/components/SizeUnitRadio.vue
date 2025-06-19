<script setup lang="ts">
import { SizeUnit } from "../types";
const props = defineProps<{
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
button {
    width: 28px;
    height: 20px;
    padding: 2px 3px;
    font-size: 14px;
}
.selected {
    background: aquamarine;
    color: white;
}
</style>
