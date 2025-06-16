<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { defineProps } from 'vue'
const props = defineProps(['workingPath'])
console.log(props)
invoke('start-analysis', { path: props.workingPath })
</script>

<template>
	<header class="top-bar">
		<input :value="props.workingPath" disabled />
	</header>
	<main></main>
</template>
