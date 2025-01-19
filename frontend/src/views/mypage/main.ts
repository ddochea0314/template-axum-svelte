import { mount } from 'svelte'
import App from './MyPage.svelte'

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app