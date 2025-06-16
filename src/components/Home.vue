<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'
import DupGroup from './DupGroup.vue'
const props = defineProps(['workingPath'])
console.log(props)
const pairs = ref([])
invoke('start_analysis', { path: props.workingPath }).then(result=>{
	pairs.value = result.pairs
})
</script>

<template>
	<header class="top-bar">
		<input :value="props.workingPath" disabled />
	</header>
	<main>
		<template v-for="pair in pairs" :key="pair.hash">
			<DupGroup v-if="pair.files.length>0" :item="pair"></DupGroup>

		</template>
	</main>
</template>

<style lang="css" scoped>
main{
	flex-grow: 1;
	padding-top: 16px;
}
</style>
