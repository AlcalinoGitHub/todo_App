import React from "react";
import styles from "./accounts.module.css"
import { useState } from "react";
import Cookies from "js-cookie" 

export interface User {
  username: string,
  password: string,
  id: number | null
}

export enum accounts {
  signin = "signin",
  signup = "signup"
}

export default function Account({type}:{type: accounts}) {
  let [error, setError] = useState("Everything is fine for now")


  const handleUsername = (event: React.ChangeEvent<HTMLInputElement>) => {
    setError("Everything is fine for now")
    let data = event.target as HTMLInputElement;
    if (data != null) {data.style.borderColor = event.target.value.length > 4 ? "green" : "red"}
    if (data != null) {if (event.target.value.length <= 4) {setError("Username to short")}} 
  } 
  
  const handlePassword = (event: React.ChangeEvent<HTMLInputElement>) => {
    setError("Everything is fine for now")
    let data =  event.target as HTMLInputElement;
    if (data != null) {data.style.borderColor = event.target.value.length > 4 ? "green" : "red"}
    if (data != null) {if (event.target.value.length <= 4) {setError("Password to short")}} 
  }

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const formData = new FormData(event.currentTarget)
    const name = formData.get("username") as string
    const password = formData.get("password") as string
    let data: User = {username: name, password: password, id: null}
    console.log(data)

    let response = await fetch(`http://localhost:8000/${type}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data)
    })
    let parsed: User | Response
    try {parsed = await response.json(); Cookies.set("auth-cookie", JSON.stringify(parsed)) ; window.location.href = "/"} catch {
      let errorMessage =type == accounts.signin ? "Account not found" : "Username Taken"
      setError(errorMessage)
    }

  }

  const getMessage = () => {
    return type == accounts.signin ?
      <div>Are you a new member? <a style={{color: "white"}} href = "/signup">Signup!</a></div> 
      :
      <div><div>Already a member? <a style={{color: "white"}} href = "/signin">Signin!</a></div></div>
  }
  
  return (
    <div className={styles.body_account}>
      <form onSubmit={(e)  =>  {handleSubmit(e)}} className={styles.form_account}>
       <h1 style={{width: "100%", textAlign:"center"}}>{type.toUpperCase()}</h1>
        <input className={styles.input} type='text' placeholder='username' name = "username" onChange={(e) => {handleUsername(e)}}/> <br/> 
        <input className={styles.input} type='password' placeholder='password' name = "password" onChange={(e) => {handlePassword(e)}}/>
        <div style = {{width: "100%", textAlign:"center"}}>{error}</div>
        <button type="submit" className={styles.button}>{type.toUpperCase()}</button>
        <div style = {{width: "100%", textAlign:"center"}}> {getMessage()} </div>
      </form>
    </div>
  )
}

