# Introduction

<div align="center">

![Keywich Icon](./images/keywich_icon.png)

</div>
<br>
<br>

Keywich is a password generator application built with Rust, [SvelteKit](https://kit.svelte.dev/),
[Skeleton UI](https://www.skeleton.dev/) and [Tauri](https://tauri.app/). The app does not store generated passwords.
Instead, it combines parameters such as the username, domain, and master password with a bit of salt and a hashing
algorithm to create reproducible passwords with the specified length and character sets.

<br>

```
 Application architecture overview      
                                                                  
 ┌───────────────────┐     ┌──────────────────┐                                     
 │ OS Keyring        │     │ Profile Database │                                     
 │                   │     │                  │                                     
 │ - Master Password │     │ - Charsets       │                                     
 └─────────┬─────────┘     │ - Domains        │                                     
           │               │ - Usernames      │                                     
           │               │ - Tags           │                                     
           │               └────────┬─────────┘                                     
           │                        │                                               
           └─────────┐    ┌─────────┘                                               
                     │    │                                  Input in, password out 
                     ▼    ▼                                  Nothing stored         
 ┌────────────────────────────────────────────┐              ┌───────────────┐      
 │                 Tauri App                  │              │    CLI App    │      
 └─────────────────────┬──────────────────────┘              └───────┬───────┘      
                       │                                             │              
                       └───────┐                 ┌───────────────────┘              
                               │                 │                                  
                               │                 │                                  
                               ▼                 ▼                                  
 ┌───────────────────────────────────────────────────────────────────────────┐      
 │                               Keywich Password                            │      
 │                                  Generator                                │      
 └─────────────────────────────────────┬─────────────────────────────────────┘      
                                       │                                            
                                       ▼                                            
                             ┌────────────────────┐                                 
                             │ Generated Password │                                 
                             └────────────────────┘                                 
```


