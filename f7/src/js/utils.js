import { io } from "socket.io-client";
const study_server_msg = "study_server_msg";

export const MSG = {
    // get_last_index: (level) => {
    //     dispatchEvent(new CustomEvent(study_server_msg, {
    //         detail: {
    //             msg: 'get_last_index', data: { level: parseInt(level) }
    //         }
    //     }));
    // },
    // get_study_list: (level) => {
    //     dispatchEvent(new CustomEvent(study_server_msg, {
    //         detail: {
    //             msg: 'get_study_list', data: { level: parseInt(level) }
    //         }
    //     }));
    // },
    // post_study: (data) => {
    //     dispatchEvent(new CustomEvent(study_server_msg, {
    //         detail: {
    //             msg: 'post_study', data
    //         }
    //     }));
    // },
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

export const socket_io_register = (token) => {
    const ioc = io("/ws", {
        auth: (cb) => {
            cb({ token: token || 'test' })
        }
    });
    ioc.on('connect', () => {
        console.log('io connect');
        window.addEventListener(study_server_msg, (data) => {
            console.log('send => ', data.detail)
            ioc.emit('study_msg', data.detail)
        }, false);
    });
    ioc.on('study_msg_resp', data => {
        console.log('recv => ', 'study_msg_resp', data);
        dispatchEvent(new CustomEvent('study_page_msg', {
            detail: data
        }));
    })
}