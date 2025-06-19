<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { Ref, ref } from 'vue'
import DupGroup from './DupGroup.vue'
import AnalysisProgress from './AnalysisProgress.vue'
const props = defineProps(['workingPath', 'config'])
const emit = defineEmits(['back'])
const analysisDone = ref(false)
type Pair = {
	hash: string
	files: string[]
}
const pairs: Ref<Pair[]> = ref([])
invoke<{ pairs: Pair[] }>('start_analysis', { path: props.workingPath, config: props.config }).then((result) => {
	analysisDone.value = true
	pairs.value = result.pairs
})

function goback() {
	emit('back')
}
</script>

<template>
	<header class="top-bar">
		<input :value="props.workingPath" disabled />
		<button @click="goback" :disabled="!analysisDone">Back</button>
	</header>
	<main>
		<AnalysisProgress v-if="!analysisDone"></AnalysisProgress>
		<template v-for="pair in pairs" :key="pair.hash">
			<DupGroup v-if="pair.files.length > 0" :root-path="workingPath" :item="pair"></DupGroup>
		</template>
		<div v-if="analysisDone && pairs.length === 0" class="no-result">
			No duplication found in this directory.
		</div>
	</main>
</template>

<style lang="css" scoped>
header.top-bar {
	display: flex;
	justify-content: space-between;
}

header.top-bar input {
	flex-grow: 1;
}

main {
	flex-grow: 1;
	padding-top: 16px;
}

.no-result {
	display: flex;
	height: 100%;
	font-size: 24px;
	align-items: center;
	justify-content: center;
}
</style>
