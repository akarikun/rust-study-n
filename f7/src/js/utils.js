import { io } from "socket.io-client";
import { f7, f7ready } from 'framework7-vue';

const study_server_msg = "study_server_msg";

export const MSG = {
    send_message: (msg, data) => {
        dispatchEvent(new CustomEvent(study_server_msg, {
            detail: {
                msg, data
            }
        }));
    },
    register_page: (fn) => {
        window.addEventListener('study_page_msg', data => {
            fn(data.detail)
        }, false);
    }
}

const dbg = () => {
    return document.querySelectorAll('link[rel="stylesheet"]').length == 0;
}

export const socket_io_register = (token) => {
    const ioc = io("/ws", {
        auth: (cb) => {
            // cb({ token: token || 'test' })
            cb({ token: token })
        }
    });
    ioc.on('connect', () => {
        console.log('io connect');
        window.addEventListener(study_server_msg, (data) => {
            if (dbg()) {
                console.log('send => ', data.detail)
            }
            ioc.emit('study_msg', data.detail)
        }, false);
    });
    ioc.on('study_msg_resp', data => {
        if (dbg()) {
            console.log('recv => ', 'study_msg_resp', data);
        }
        dispatchEvent(new CustomEvent('study_page_msg', {
            detail: data
        }));
    })
}

export const format_title = (form) => {
    let level = ['', 'N1', 'N2', 'N3', 'N4-N5'][form.level];
    return form.id > 0 ? `编辑(${level} - ${form.id})` : `录入`
}

export const format_level = (level) => {
    return ['', 'N1', 'N2', 'N3', 'N4-N5'][level];
}

export const alert = (msg, fn) => {
    fn = fn || function () { };
    f7.dialog.alert(msg, '日语学习', () => { fn(); });
}

export const go = (url) => {
    f7.router.navigate(url)
}