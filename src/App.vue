<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window'
import Button from 'primevue/button';
import ButtonGroup from 'primevue/buttongroup';

import ToggleSwitch from 'primevue/toggleswitch';

import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';

const autoboot = ref(false)
async function initAutostart() {
  autoboot.value = await isEnabled()
}
initAutostart()

function parseCSV(data) {
  var lines = data.trim().split('\n')
  const headers = lines[0].trim().split(',');
  const result = lines.slice(1).map(line => {
        const values = line.trim().split(',');
        const dict = {};
        headers.forEach((header, index) => {
            dict[header] = values[index];
        });
        return dict;
    });
    return result;
}

const time = ref(new Date())
function getCurrentDateAndWeekday() {
    const now = time.value;
    const year = now.getFullYear();
    const month = now.getMonth() + 1;
    const day = now.getDate();
    const weekdays = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'];
    const weekday = weekdays[now.getDay()];
    const formattedDate = `${year}年${month.toString().padStart(2, '0')}月${day.toString().padStart(2, '0')}日`;

    return { date: formattedDate, weekday: weekday };
}

var currentDateAndWeekday = getCurrentDateAndWeekday();
const date = ref(currentDateAndWeekday.date)
const weekday = ref(currentDateAndWeekday.weekday)
function isCurrentTimeBetween(startTime, endTime) {
    const now = time.value;
    const currentMinutes = now.getHours() * 60 + now.getMinutes();
    function parseTimeToMinutes(timeStr) {
        const [hours, minutes] = timeStr.split(':').map(Number);
        return hours * 60 + minutes;
    }
    const startMinutes = parseTimeToMinutes(startTime);
    const endMinutes = parseTimeToMinutes(endTime);
    return currentMinutes >= startMinutes && currentMinutes <= endMinutes;
}
const record = ref([])
const appWindow = getCurrentWindow();
async function readCSV() {
  var content = await invoke('read_csv');
  record.value = parseCSV(content)
}
readCSV()
const settings_panel = ref(false)
onMounted(() => {
  setInterval(() => {
    time.value = new Date()
  }, 1000)
  setInterval(() => {
    currentDateAndWeekday = getCurrentDateAndWeekday();
    date.value = currentDateAndWeekday.date;
    weekday.value = currentDateAndWeekday.weekday;
  }, 60000)
})
</script>

<template>
  <div data-tauri-drag-region 
    class="flex flex-col h-screen w-screen bg-slate-200 select-none" 
    @contextmenu.prevent="">
    <div data-tauri-drag-region  class="flex flex-row">
      <div data-tauri-drag-region class="flex-1"/>
      <ButtonGroup>
        <Button icon="pi pi-cog" 
          outlined 
          class="!rounded-none !border-0 !h-10 !w-10"
          @click="settings_panel = !settings_panel"/>
        <Button icon="pi pi-times" 
          severity="danger"
          class="!rounded-none !h-10 !w-10"
          @click="appWindow.close()"/>
      </ButtonGroup>
    </div>
    <div data-tauri-drag-region v-if="!settings_panel" class="p-2 flex flex-col items-center" >
      <div data-tauri-drag-region class="text-1xl font-bold text-slate-800">{{ date }}</div>
      <div data-tauri-drag-region class="text-4xl font-bold text-slate-800">{{ time.toLocaleTimeString() }}</div>

      <div data-tauri-drag-region class="bg-black h-0.5 w-full"/>
      <template v-for="item in record" >
        <div v-if='item[weekday]!="-"'
          data-tauri-drag-region 
          :class="`m-1 text-xl items-center ${isCurrentTimeBetween(item['start'], item['end']) ? 'font-bold' : ''}`">
          {{ item[weekday] }}
        </div>
        <div  v-else data-tauri-drag-region class=" bg-slate-500 h-0.5 w-5/6"/>
      </template>
      
    </div>
    <div data-tauri-drag-region v-else class="w-full p-2 flex flex-col items-center gap-2">
        <div data-tauri-drag-region class="flex flex-row w-full items-center">
          <div data-tauri-drag-region>编辑课程</div>
          <div data-tauri-drag-region class="flex-1"></div>
          <Button data-tauri-drag-region icon="pi pi-pen-to-square" 
            class=" h-10 w-10"
            @click="invoke('open_csv')"/>
        </div>
        <div data-tauri-drag-region class="flex flex-row w-full items-center">
          <div data-tauri-drag-region>自启动</div>
          <div data-tauri-drag-region class="flex-1"></div>
          <ToggleSwitch data-tauri-drag-region 
            v-model="autoboot" 
            @change="()=>{
              autoboot? enable() : disable()
              v = isEnabled()
              console.log(v)
              }"/>
        </div>
        <div data-tauri-drag-region class="flex flex-row w-full items-center">
          <div data-tauri-drag-region>返回</div>
          <div data-tauri-drag-region class="flex-1"></div>
          <Button data-tauri-drag-region icon="pi pi-arrow-left" 
            class="h-10 w-10"
            @click="settings_panel = !settings_panel"/>
        </div>
      </div>
  </div>
</template>
