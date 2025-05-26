import { ViewMgr } from "../ui/viewmgr";

const viewMgr = new ViewMgr();

document.getElementById("button-set-view1").onclick = () => {
    viewMgr.setPrimaryView("view1");
};
document.getElementById("button-set-view2").onclick = () => {
    viewMgr.setPrimaryView("view2");
};
