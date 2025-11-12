/** @type {import('./$types').PageLoad} */
import { invoke } from '@tauri-apps/api/core';

export async function load({params}){ 
	const person = await invoke("get_person");
	   return {        
		page: params.page,
		videoPath: `/videos/${person}/${params.page}.mp4`,
	      }
	}
