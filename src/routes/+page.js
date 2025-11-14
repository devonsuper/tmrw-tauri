/** @type {import('./$types').PageLoad} */
import { invoke } from '@tauri-apps/api/core';

export async function load({params}){ 
  const person = await invoke("get_person");
     return {        
    videoPath: `http://127.0.0.1:8080/videos/${person}/idle.mp4`
        }
  }