<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const props = defineProps(['item','rootPath'])
function removeItem(path:string,parent:String[]){
    invoke('remove_file',{rootPath:props.rootPath,relPath:path}).then(()=>{
        let index = parent.indexOf(path)
        if(index> -1){
            parent.splice(index,1)
        }
    })
}
function revealItem(path:string){
	invoke('reveal_file_in_explorer',{rootPath:props.rootPath,relPath:path})
}
</script>

<template>
	<section>
		<p>
			<span class="hash">{{ item.hash }}</span>&nbsp;&nbsp;<span class="size">{{ item.size }}</span>
		</p>
		<ul>
			<li class="file-item" v-for="child in item.files">
				<!-- <input type="checkbox"> -->
				<span class="file-name">{{ child }}</span>
				<div class="ctrl">
					<div @click="revealItem(child)" title="Reveal in file explorer" class="reveal">REV</div>
					<div @click="removeItem(child,item.files)" title="Remove file" class="delete">DEL</div>
				</div>
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
	font-family: ui-monospace, 'Cascadia Mono', 'Segoe UI Mono', 'Roboto Mono', 
               'Fira Mono', 'Droid Sans Mono', 'Source Code Pro', Menlo, 
               Monaco, 'Ubuntu Mono', Consolas, monospace;
	font-weight: bold;
}
span.size{
	color: gray;
	font-size: 14px;
}
span.file-name{
	font-size: 14px;
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

/* li.file-item:hover{
    background: rgba(0,0,0,0.2);
} */
 .ctrl{
	display: flex;
 }
.ctrl div{
	font-size: 12px;
	padding: 2px 8px;
    cursor: pointer;

}
.ctrl div:hover{
	font-size: 12px;
	box-shadow: 0 0 2px rgba(0,0,0,0.2);
	background-color: white;
}
.ctrl div.delete{
    color: red;
}
.ctrl div.delete:hover{
    color: white;
	background-color: red;
}
.ctrl div.reveal{
	color: black;
}
.ctrl div.reveal:hover{
	color: white;
	background: black;
}
</style>
