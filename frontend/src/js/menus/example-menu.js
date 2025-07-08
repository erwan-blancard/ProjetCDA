import { ViewMgr } from "../ui/viewmgr";
import { initMenuParticles } from './menu_particles.js';

const viewMgr = new ViewMgr();

window.addEventListener('DOMContentLoaded', () => {
  initMenuParticles();
});

document.getElementById("button-set-view1").onclick = () => {
    viewMgr.setPrimaryView("view1");
};
document.getElementById("button-set-view2").onclick = () => {
    viewMgr.setPrimaryView("view2");
};
