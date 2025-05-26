import gsap from "gsap";

export const SHOW_HIDE_DELAY = 0.2;
export const SHOW_ANIM_CLASSNAME = "show";
export const HIDE_ANIM_CLASSNAME = "hide";


// Simple class to show / hide HTML elements (considered as "views") of the page.
export class ViewMgr {
    /** @type {Array<HTMLElement>} */
    primaryViews = [];
    currentPrimaryView = -1;

    constructor(primaryViews=collectViews()) {
        this.primaryViews = primaryViews;

        this.primaryViews.forEach(view => {
            disableInteractions(view);
        });

        if (this.primaryViews.length >= 1) {
            this.currentPrimaryView = 0;
            const view = this.primaryViews[this.currentPrimaryView];
            enableInteractions(view);
            showElementAnimated(view);
        }
    }

    setPrimaryView(view) {
        if (typeof(view) == "number") {
            if (view === this.currentPrimaryView)
                return;
            this.#changePrimaryView(view);
        } else if (typeof(view) == "string") {
            const viewElement = document.getElementById(view);

            const index = this.primaryViews.indexOf(viewElement);

            if (index !== -1)
                this.#changePrimaryView(index);

        } else {
            if (view === this.primaryViews[this.currentPrimaryView])
                return;
            
            const index = this.primaryViews.indexOf(view);

            if (index !== -1)
                this.#changePrimaryView(index);

        }
    }

    #changePrimaryView(index) {
        if (this.currentPrimaryView != -1){
            const current = this.primaryViews[this.currentPrimaryView];
            hideElementAnimated(current);
        }

        this.currentPrimaryView = index;

        const new_view = this.primaryViews[index];
        setTimeout(() => {
            showElementAnimated(new_view);
        }, SHOW_HIDE_DELAY * 1000);
    }

    /**
     * Function used to show (animated) an element that is not in the view list.
     * It may be used for elements such as side panels.
    */
    showPanel(element, fromRight=false) {
        const startFrom = fromRight ? element.offsetWidth : -element.offsetWidth;
        gsap.timeline({defaults: {duration: SHOW_HIDE_DELAY},
            onComplete: () => { enableInteractions(element); },
            onStart: () => { disableInteractions(element); }}).fromTo(element,
                {
                    offsetLeft: startFrom
                }, {
                    offsetLeft: 0
                });
    }

    /**
     * Function used to hide (animated) an element that is not in the view list.
     * It may be used for elements such as side panels.
    */
    hidePanel(element, fromRight=false) {
        const startFrom = fromRight ? element.offsetWidth : -element.offsetWidth;
        gsap.timeline({defaults: {duration: SHOW_HIDE_DELAY},
            onStart: () => { disableInteractions(element); }}).fromTo(element,
                {
                    offsetLeft: startFrom
                }, {
                    offsetLeft: 0
                });
    }

}


export function disableInteractions(element) {
    element.style.pointerEvents = "none";
    element.style.userSelect = "none";
}

export function enableInteractions(element) {
    element.style.pointerEvents = "initial";
    element.style.userSelect = "initial";
}

export function showElementAnimated(element) {
    enableInteractions(element);
    element.style.display = "initial";
    element.classList.remove(HIDE_ANIM_CLASSNAME);
    element.classList.add(SHOW_ANIM_CLASSNAME);
}

export function hideElementAnimated(element) {
    disableInteractions(element);
    element.classList.remove(SHOW_ANIM_CLASSNAME);
    element.classList.add(HIDE_ANIM_CLASSNAME);
    // hide after anim end
    setTimeout(() => {
        element.style.display = "none";
    }, SHOW_HIDE_DELAY * 1000);
}


/**
 * Function to collect views of the document and put them in an array.
 * Any element with the class "view" is collected.
*/
export function collectViews() {
    const documentViews = document.getElementsByClassName("view");
    const views = new Array(documentViews.length);

    for (let i = 0; i < documentViews.length; i++)
        views[i] = documentViews[i];

    return views;
}
