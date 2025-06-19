<template>
    <div class="labeled-input">
        <label for="label">{{ label }}</label>
        <input
            :type="type"
            :placeholder="placeholder"
            :value="modelValue"
            @input="updateValue"
            @change="emit('change', $event)"
            @blur="emit('blur', $event)"
        />
    </div>
</template>

<script setup>
const props = defineProps({
    label: String,
    type: String,
    placeholder: String,
    modelValue: String,
});

const emit = defineEmits(["update:modelValue", "change", "blur"]);

function updateValue(event) {
    emit("update:modelValue", event.target.value);
}
</script>

<style lang="css" scoped>
.labeled-input {
    align-items: center;
    position: relative;
    margin-top: 12px;
}

label {
    position: absolute;
    top: -12px;
    left: 4px;
    font-size: 12px;
    color: rgb(30, 30, 30);
}

input {
    width: 100%;
    height: 100%;
}
</style>
