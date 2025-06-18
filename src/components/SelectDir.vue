

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
const emit = defineEmits(['path-selected'])
async function chooseDirectory() {
    open({
        multiple: false,
        directory: true,
    }).then(path => {
        console.log(path)
        if(path){
            emit('path-selected', path)
        }
    }).catch(e => {
        console.log(e)
    });
}
</script>

<template>
    <div class="home">

        <div class="mask">

        </div>
        <button @click="chooseDirectory">Choose a directory...</button>
    </div>
</template>

<style lang="css" scoped>
.home {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-grow: 1;
}

button {
    font-size: 24px;
    width: 60%;
    z-index: 1;
    height: 80px;
}

.mask {
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 0;
}
</style>