export const dispatchMessage = (detail) => {
    dispatchEvent(new CustomEvent('study_msg', {
        detail
    }));
}