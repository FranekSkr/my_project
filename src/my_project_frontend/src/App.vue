<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let greeting = ref('');
let chat= ref('');

async function handleSubmit(e) {
 e.preventDefault();
 const target = e.target;
  const name = target.querySelector('#name').value;
  await my_project_backend.greet(name).then((response) => {
    greeting.value = response;
  });
  const newMsg = target.querySelector('#newMsg').value;
  await my_project_backend.add_msg(newMsg);
  await getChat();
}

async function getChat() {

  chat.value =  await my_project_backend.get_chat();
  
}
getChat();
    
</script>

<template>
@@ -19,10 +26,13 @@ async function handleSubmit(e) {
<br />
<br />
<form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <label for="newMsg">Enter your newMsg: &nbsp;</label>
      <input id="newMsg" alt="newMsg" type="text" />
<button type="submit">Click Me!</button>
</form>
    <section id="greeting">{{ greeting }}</section>
    <button @click = "getChat">refresh</button>
    <section id="chat">
      <div v-for="msg in chat ">{{ msg }}</div>
    </section>
</main>
</template>
