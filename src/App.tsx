import React, { useState, useEffect } from "react";
import styles from "./app.module.css"
import {User} from "./Accounts"
import Cookies from "js-cookie";

function NotLoggedIn() {
    return (
        <>
        <div style={{height: "95vh", width: "95vw"}}>
            <div className={styles.notlogged}>
                <h1>You do not seem to be logged in :(</h1>
                <div className={styles.notloggedoptions}>
                    <div onClick={() => {window.location.href = '/signin'}}  className={styles.notloggedbutton} > 
                        <div style={{color: "white"}}> signin </div> 
                    </div> 
                    <div onClick={() => {window.location.href = '/signup'}} className={styles.notloggedbutton}>
                        <div style={{color: "white"}}> signup </div>
                    </div>
                </div>
            </div>
        </div>
        </>
    )
}


interface Task {
    name: string,
    description: string,
    task_id?: number,
    user_id: number,
    due: string
    completed: boolean
}

interface TaskRequest {
    user: User,
    task: Task
}

interface TaskDelete {
    user: User,
    task_id: number
}

function CreateTaskMenu() {

    return (
    <>
        <input className={styles.create_input} type = "text" name = "name" placeholder="name"></input> <br/>
        <input className={styles.create_input} type = "text" name = "description" placeholder="description"></input> <br/>
        <input className={styles.create_input} type = "date" name = "due"></input> <br/>
        <button className={styles.create_button} type = "submit">Create Task</button>
    </>
    )
}

function SeeTask(task: {task:Task, user: User, onDelete: Function}) {
    let [completed, setCompleted] = useState(false)
    useEffect(() => {
        setCompleted(task.task.completed)
    }, [task])

    async function markCompleted() {
        setCompleted(completed == true ? false : true)
        task.task.completed = completed == true ? false : true
        let taskEdit: TaskRequest = {
            task: task.task,
            user: task.user
        }
        console.log(taskEdit)
        let response = await fetch("http://localhost:8000/edit_task", {
            body: JSON.stringify(taskEdit),
            method: "POST",
            headers: {"Content-Type": "application/json",}
        })
        let data = await response.json()
        console.log(response, data)
    }

    async function deleteTask() {
        if (!task.task.task_id ) {return}
        let task_delete: TaskDelete = {
            user: task.user,
            task_id: task.task.task_id
        }
        let response = await fetch("http://localhost:8000/delete_task", {
            body: JSON.stringify(task_delete),
            method: "DELETE",
            headers: {"Content-Type": "application/json",}
        })
        task.onDelete()
        console.log(response)
    }
    let text_decoration = completed ? "line-through" : "none"

    return (
        <div className={styles.see_task} style={{textDecoration: text_decoration}}>
            <div>Name: {task.task.name}</div>
            <div>Desc: {task.task.description}</div>
            <div>Due: {task.task.due}</div>
            <div>Completed: {JSON.stringify(completed)}</div>
            <div>
                <button className={styles.action_button} onClick={deleteTask} >🗑️</button>
                <button className={styles.action_button} onClick={markCompleted}>{completed == true ? "❌" : "✔️"}</button>
            </div>
        </div>
    )
}

export default function App() {
    let [tasks, setTasks] = useState<Task[]>([])
    let user_data: string | undefined = Cookies.get("auth-cookie")
    if (user_data == undefined) {return <NotLoggedIn/>}
    let user: User = JSON.parse(user_data);

    async function handleCreateTask(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault()
        const formData = new FormData(event.currentTarget)
        const name = formData.get("name") as string
        const description = formData.get("description") as string
        const due = formData.get("due") as string
        if (name == "") {return}
        let new_task: Task 
        if (user.id != null){new_task = {name, description, due, user_id: user.id, completed: false}} else {return} 
        let new_request: TaskRequest  = {user: user, task: new_task}
        console.log(new_request)
        let response = await fetch("http://localhost:8000/create_task", {
            body: JSON.stringify(new_request),
            method: "POST",
            headers: {"Content-Type": "application/json",}
        })
        console.log(response)
        let data = await response.json()
        console.log(data)
        getTasks()
    }

    async function getTasks() {
        let response = await fetch("http://localhost:8000/get_tasks", {
            body: JSON.stringify(user),
            method: "POST",
            headers: {"Content-Type": "application/json",}
        })
        let data = await response.json()
        setTasks([])
        setTasks(data)
        console.log(data)
    } 
    useEffect(() => {

        getTasks();
    }, [])
    return (
        <>
            <button onClick = {() => {Cookies.remove("auth-cookie"); window.location.href = "/"}} >Logout</button>
            <h1>Welcome {user.username}</h1>
            <div style={{display: "flex", flexDirection: "row"}}>
                <form className={styles.create_form} onSubmit={(e) => {handleCreateTask(e)}}> <CreateTaskMenu/> </form>
                <div className={styles.task_container}>
                    {tasks.map((x, index) => {return <SeeTask key = {index} task = {x} user = {user} onDelete={getTasks}/>})}
                </div>
            </div>
        </>
    )
}