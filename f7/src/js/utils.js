export const study_page_msg = "study_page_msg"
export const study_server_msg = "study_server_msg"

const dispatchMessage = (msg, detail) => {
    dispatchEvent(new CustomEvent(msg, {
        detail
    }));
}
export const dispatchServerMessage = (detail) => {
    dispatchMessage(study_server_msg, detail);
}
export const dispatchPageMessage = (detail) => {
    dispatchMessage(study_page_msg, detail);
}