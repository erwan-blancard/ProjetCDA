import { ViewMgr } from '../ui/viewmgr';

const mainview = document.getElementById("view1");
const view2 = document.getElementById("view2");

const viewMgr = new ViewMgr([mainview, view2]);

document.getElementById("button-set-view1").onclick = () => {
    viewMgr.setPrimaryView("view1");
};
document.getElementById("button-set-view2").onclick = () => {
    viewMgr.setPrimaryView("view2");
};
