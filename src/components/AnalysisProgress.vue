<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

let stages = ['ingestion', 'quick-filter', 'hash']
let stageIndex = ref(0)

listen('ingestion-done', () => {
	stageIndex.value = 1
})
listen('quick-hash-done', () => {
	stageIndex.value = 2
})
listen('full-hash-done', () => {
	stageIndex.value = 3
})
</script>

<template>
	<div class="progress">
		<ul>
			<li v-for="[index, stage] in stages.entries()" class="stage" :class="{ active: stageIndex == index, done: stageIndex > index }">{{ stage }}</li>
		</ul>
	</div>
</template>

<style scoped>
.progress ul {
	display: flex;
	width: 100%;
	--color: aquamarine;
}
.progress ul li {
	padding: 4px 10px;
	font-size: 12px;
	color: rgb(51, 51, 51);
	background-color: white;
	flex-grow: 1;
	text-align: center;
	vertical-align: top;
	/* height: 4px; */
}
.progress ul li.active {
	background: linear-gradient(45deg, #fff 20%, var(--color) 40%, var(--color) 60%, #fff 80%);
	background-size: 200% auto;
	animation: shine 1s linear infinite;
}
.progress ul li.done {
	background-color: var(--color);
}
@keyframes shine {
	/* from {
        background-position: 0% center;
    } */
	to {
		background-position: -200% center;
	}
}
</style>
