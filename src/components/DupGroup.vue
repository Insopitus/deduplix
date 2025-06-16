<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

defineProps(['item'])
function removeItem(path:string,parent:String[]){
    invoke('remove_file',{path}).then(res=>{
        let index = parent.indexOf(path)
        if(index> -1){
            parent.splice(index,1)
        }
    })
}
</script>

<template>
	<section>
		<p>
			<span class="hash">{{ item.hash }}</span> <span>- {{ item.size }}</span>
		</p>
		<ul>
			<li class="file-item" v-for="child in item.files">
				<!-- <input type="checkbox"> -->
				<span>{{ child }}</span>
                <span @click="removeItem(child,item.files)" class="delete">&times;</span>
			</li>
		</ul>
	</section>
</template>

<style lang="css" scoped>
p {
	display: block;
	background-color: aquamarine;
	padding: 4px 8px;
    margin-bottom: 4px;
}
span.hash {
	font-family: Consolas, monospace;
	font-weight: bold;
}
ul {
	margin: 0;
}
li {
	margin-left: 12px;
}
input[type='checkbox'] {
	box-shadow: none;
}
li.file-item{
    padding: 0 4px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}
span.delete{
    cursor: pointer;
    color: red;
}
li.file-item:hover{
    background: rgba(0,0,0,0.2);
}
</style>
