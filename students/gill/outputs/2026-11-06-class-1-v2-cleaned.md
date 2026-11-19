# 2026-11-06-class-1: cleaned transcript

Cleaned from `transcripts/2026-11-06-class-1.md` for extraction. Speaker labels are surname-comma-firstname, filler and greetings removed, product names corrected. Every quotation in `2026-11-06-class-1-v3.md` is checked against this file.

Gill, Mungo [0:40]: I think I'm pulling behind every other student at the moment. I'm a beginner.

Falco, Vinnie [0:41]: You've kind of gotten some ambient knowledge. Cynthia, let me know when I should start.

Falco, Vinnie [1:01]: I made the repo. That repo came together quick. The wonders of AI.

Kaleka, Greg [1:20]: I see there were a couple of things we were supposed to do before class. I did not have time.

Falco, Vinnie [1:25]: You mean read those documents? I don't know, that's not realistic.

Falco, Vinnie [2:28]: What I'm gonna do is start from the most exciting thing, and I'm gonna work backwards, to get you all interested.

Falco, Vinnie [2:46]: We've created this repository called Runvil Classroom, and I'm trying a new approach to bring the team up to speed. This approach is gonna have more documentation, more artifacts, more participation, and I'm gonna call on everyone to help me help you, and help you help me.

Falco, Vinnie [3:08]: The lesson plan for today is here in the repo, and there's some documents that you're gonna wanna read. One of them is called Saving All Gathered Expertise, and this is a paper about how to interview people.

Falco, Vinnie [3:29]: To get them to tell stories, and capture what I call tacit knowledge. That is experiences and skills and judgment that somebody has because they've been doing something for a long time, but they acquired it from experience, and it's not necessarily written down. There's no rules for it.

Falco, Vinnie [3:51]: That paper describes a system for automating that process of capturing that knowledge with interviews. Then I made this other document, how an agent harness works, and this one is specially designed to tell you the nuts and bolts of how the parts of an agent harness that Runvil cares about work.

Falco, Vinnie [4:09]: It's been curated to help you understand a generic harness. Once you understand this, you'll understand how every agent harness works, like Cursor, Claude Code, ChatGPT web or whatever. This is all valuable information, and it's applicable to Runvil.

Falco, Vinnie [4:27]: Then there are 3 key documents: Knowledge Vision, Runvil vision, Workshop vision.

Falco, Vinnie [4:48]: Runvil is an explanation of the prompting language: why do we have it, what's it for, why is it better. Workshop vision explains the Runvil Workshop, the IDE. Knowledge vision is the thing that I'm gonna tell you all about. Some people know about it, some people don't, but this is the thing that we're working towards. Knowledge vision is what comes after Runvil.

Falco, Vinnie [5:16]: Here's the idea: we use Runvil to implement some of the prompts that I've already created. Will knows about this Talktron. Talktron interviews you, and it creates a transcript, and it gets you to talk. Then I have another tool called the Pedagogue, which creates an interactive learning program: it takes information and it teaches you. My prototypes are a little bit crude, but they're a proof of concept, and we can turn them into something real.

Falco, Vinnie [5:50]: I'm creating a for-profit company. It's called the Knowledge Capture Corporation. There's 3 products. One of them is called on-site knowledge capture. That means we go to someone's company, and we set up maybe a docker container in their network, or we provide a cloud service, and they pay a subscription fee, and then we design structured interviews, either human or AI assisted, where their employees who know things get interviewed, they relate their experiences, and those turn into transcripts.

Falco, Vinnie [6:28]: The transcripts get the knowledge extracted into experiences and principles, which could be design principles, and then that goes into a database. The database is used to power an interactive learning program. It's like a teacher that's designed just for you. It goes at your pace. If you're a fast learner, it goes faster. If you're slow, it slows down.

Falco, Vinnie [6:53]: This prompt is a prototype of that tool, and it has these learning techniques. The prompt is powered with actual methods that are effective in helping students learn.

Falco, Vinnie [7:14]: If you look at the prompt, you can see that it's got Never reveal the answer. Then this is what to do, and then you can see some of the teaching rules.

Falco, Vinnie [7:33]: When it creates the tutor, there are these rules that it follows, and they're designed to help with adaptive learning. If a student makes like 2 or 3 mistakes in a row, then it backs off, and then maybe it goes to a simpler topic so they can build confidence.

Falco, Vinnie [8:03]: This generates revenue. This is for profit, there's no nonprofit here, we're selling this, we're making money. That's product #1. Product #2 is a tool that's autobiographical. It's the same technology: somebody gets interviewed, but instead of talking about their business, it's personal. Maybe they're talking about their life, maybe they're dying or they want to leave something for their grandchildren, so that they can know them. People can create their own autobiography and it captures that knowledge. Maybe you wanna capture a CEO who's retiring; you want to get his stories about how he ran the company.

Falco, Vinnie [8:43]: The product that gets created here is not a virtual simulation of the person, that would be weird. Instead, it's a trusted virtual confidant. It's an AI persona who knows the person, who can answer questions about the person. Hey, what was Vinnie like when he was up at Ripple? Or, hey, what was my grandfather like after World War II, when he was in the motor pool? It knows what it knows.

Falco, Vinnie [9:16]: Or you might ask, what was Steve Jobs' approach to making new iPhones not suck? So there's corporate value there as well, but the point is that this is a tool, and this is also a product. It would be a cloud service and it would be automated. You go to the website, you do your interviews, it creates it, and then you pay a monthly fee, you get access to it, or you can buy it and download the data. That's the 2nd product.

Falco, Vinnie [9:42]: The 3rd product is a knowledge marketplace. It's like YouTube, but instead of videos, you have interactive learning programs. It looks just like YouTube: there's a thumbnail, but when you click on it, instead of getting a video, you get whatever the author decided. They might have an avatar with a voice, or something that looks like a textbook, or something that looks like a slideshow, but it's a teaching program. It teaches you about a subject, and it's interactive, and it adapts to you.

Falco, Vinnie [10:12]: It figures out how smart you are and how much you know, and it accelerates at your pace or it slows down. The person who creates that content doesn't need to worry about that; that's taken care of by our system. There's a subscription fee, and people who create good content, we're gonna compensate them. They're gonna get a fraction of the views, something to incentivize the content creators, but it's all based on learning. Whoever creates the most popular learning programs, they're gonna get paid the most.

Falco, Vinnie [10:42]: 10% of all people who subscribe and use these programs get to use, at random, 10% of the learning programs for free, as long as they review afterwards and leave a comment. So that guarantees that we always get ratings, and that they're randomly distributed, and they can't be gamed because they're random.

Falco, Vinnie [11:08]: All of these products are built on Runvil. I wanna create these things, and my goal is to have this company either get bought out or go public. Which will be fun for all of us. That's the vision and that's the Knowledge Capture Corporation.

Falco, Vinnie [11:29]: Now taking a step back: you've seen Runvil, you've seen Workshop. What's it all about? Runvil is the prompting language, and I'm not gonna bore you with the details, but I'm happy to answer questions. The idea with Runvil is that the author of the prompt decides which aspect has the authority.

Falco, Vinnie [12:05]: For example, you can decide that the code has the authority to orchestrate. The model starts out with the authority to perform inference. If you want to have a classifier, which is a feature we're gonna have to have, then you can delegate classification tasks to the classifier, but you're not forced to do that. You can opt in to having the model orchestrate. You can do that throughout the entire prompt, or you could do it in part of the prompt. For example, you can have some Lua orchestration at first to do some research, and then you can present the model with a bunch of documents and internet links, and then you can let it orchestrate that. The power of the language is that you have complete control. The author can offer tools to the model selectively: in one case you can say you can have these tools, and then in another section, you can say, no, you can only have these tools.

Falco, Vinnie [12:48]: You can even invent tools; they're called ad hoc tools. You could write an ad hoc tool and offer it to the model, and then it can call it, and it can do things. It's a very powerful way to write a prompt. In order to understand how all that works, we have these tutorials. Really, I think it's just one. This tutorial, how an agent harness works, goes back to what I was saying.

Falco, Vinnie [13:13]: This will help you understand that when a Runvil prompt runs, this is what's running, this is how it's going.

Falco, Vinnie [13:30]: I've told people this story, but I think it's worth saying again. We have AI notes and transcription on, so I don't have to repeat it. We'll just put it in a document. I like Cursor.

Falco, Vinnie [13:47]: I wanna keep using it, and I never wanna go back to the old way of doing things. I don't ever want to run Visual Studio again. I don't wanna have to switch IDEs. But the problem is that I don't own Cursor, and the people who own it, they like making decisions for me that I don't necessarily agree with. For example, in Cursor, when you type in a prompt, that prompt goes to their server to enrich the context, and then it comes back and then it goes to Anthropic. I don't want that, because I want to enrich the content my way.

Falco, Vinnie [14:21]: Sometimes I want to enrich the context one way, sometimes I want to enrich it another way. Sometimes when the agent compacts, I want some things to stay, I want some things to go. It all depends on the prompt. The Mentographist might not need to keep a whole big history, it might only need to keep the last two stories. So when it compacts, it could compact more frequently and keep the token costs down. Nobody's doing that, and this is an opportunity for us.

Falco, Vinnie [14:51]: The point of the IDE is to have something that understands Runvil. When you have a prompt, I want to be able to load up the prompt, press a keyboard shortcut and run it, and then maybe set a breakpoint. You can set a breakpoint at a certain spot, and then you can inspect it. You can say, what's the state of this prompt? What does the store look like? What do the files look like? What did it do? What do the variables look like? And then maybe we say we wanna change something, and then we wanna run it again.

Falco, Vinnie [15:21]: But we don't want to do all that inference again. Some of these reporting tools that I wrote can take 20 minutes or 30 minutes. They go on the internet, they download all these web pages, they look through them, they extract data. If we have to do that every time we want to make a change, that sucks. Runvil is designed so that when the prompt runs, it records everything that it does: every tool call, every function call in Lua, everything that happens, every interaction with the model.

Falco, Vinnie [15:46]: It can be recorded, and then if you want to have a breakpoint and you want to run the prompt over again, we can replay that recorded data, and it will get you right back to where you were at that breakpoint. Then you can try something different, like change a constant. Say you have a 10 step pipeline.

Falco, Vinnie [16:15]: You're optimizing the prompts, and you've got it all the way to step 6: steps 1 through 6 are good, but step 7, the prompt is maybe not so great, maybe it's hallucinating a little bit, and you wanna fix it. You don't wanna run steps 1 through 6 over and over again, cause that could take half an hour, it could take 2 hours. You wanna be able to start over with a different prompt for step 7, and just run that again on that same data. Anyone have any questions so far?

Falco, Vinnie [16:46]: OK, so that's the Workshop, and it's gonna be integrated with Runvil. Now let's talk about how we're gonna do these lessons.

Falco, Vinnie [17:03]: This is a new idea. I'm not gonna pretend that I know what I'm doing. So if you have input, if you would like to learn a different way, or if you have some suggestions or some advice, I'm happy to hear it.

Falco, Vinnie [17:20]: My principle is I wanna try to record everything in a transcript. Whenever we have a session, we'll have the transcript. It'll go in here. Everyone could read it, and then the AGENTS.md kind of cleans it up, if there's any profanity or stuff like that.

Falco, Vinnie [17:38]: Then we need to transcribe it and turn it into minutes. I had a conversation with Cynthia about how we were gonna do this classroom thing, so you can see there's some minutes here. 2 participants, that's me and Cynthia.

Falco, Vinnie [17:55]: We've got all this nice stuff over here that you can look at: 3 planned products, and that turned into the document. What I'm envisioning for us is we have a session where I'll talk about something, you'll ask me questions, and that'll all go in the transcript, and then afterwards we can mine those transcripts for information.

Falco, Vinnie [18:19]: Then we can use that to create documentation for the system. Now, here's the interesting thing. The pipeline of knowledge capture that I wanna build for the Knowledge Capture Corporation is actually the process that we need to use for Runvil.

Falco, Vinnie [18:39]: It's the same problem. I made a whole bunch of design decisions, at 2 a.m. on a Saturday morning. They're not written down, and we wanna know what they are. I have information about the choices that I made, and nobody knows them, so no one can tell if the code is doing the right thing. No one can tell if some implementation is missing. So your job is to get it out of me, and that is the same problem that the Knowledge Capture Corporation has.

Falco, Vinnie [19:10]: The pipeline is: you interview the subject who has tacit knowledge. We create the transcript, we put it in the transcripts directory, and then we use what's called an extraction prompt to take that transcript and turn it into something actionable. You have seen the Scribe tool. It takes a set of minutes, and it turns it into that document that you saw. That's the Scribe tool. But this might not be perfect.

Falco, Vinnie [19:46]: Because this just tells you a summary of the conversation. What we really want is a prompt that will extract the knowledge. The instructor wants daily results in the repository: that's not knowledge, nobody cares about that, that's not something that we're teaching, that's just a statement. We wanna have a prompt that extracts this.

Falco, Vinnie [20:09]: In order for everyone to gain experience and to gain skill, everyone is gonna write their own prompt to process the minutes and create the design document their way, whatever they think is important. You'll experiment, you'll work on your prompt, and then it'll go in the student's directory. Everyone's got a directory.

Falco, Vinnie [20:36]: I would like each of you who's participating in the class, after this session, to make a prompt that turns the transcript into design principles. I haven't given you any design principles yet, but I will, and then you'll look at it, you'll adjust your prompt, you'll get it to how you want it as best as you can. Don't waste too much time on it.

Falco, Vinnie [20:59]: Put that in a pull request and merge it. Everyone has to merge their own commits. I don't want to be sitting there having to review anything. All this stuff just goes in. I've given everyone write access so we can move fast. Every day or every week, we'll get better and we'll see whose prompts are more useful. I don't know ahead of time what to extract and how to present it, but at all times, our goal will be to present the information in a way that people can learn it.

Falco, Vinnie [21:31]: There's that. Now the next thing is, we want to convert the Runvil prompts.

Falco, Vinnie [21:48]: We want to convert some of the tools-public prompts, we want to turn them into Runvil prompts. I've alluded to this with some people, I forget who, but now we want to do it for real. So I would like everyone to choose a prompt.

Falco, Vinnie [22:08]: Unfortunately, we don't have enough. Jason and Glenn are exempt cause you already have your own prompt, which was huge. Because we have so many new people, Cynthia and Glenn, what we might want to consider is showing them a little preview of Agora.

Falco, Vinnie [22:34]: Show them what the idea is, cause it's a really cool idea, and the last time they had some very impressive results. That kind of shows you what you can do with Runvil and the flavor of the type of products that we're building. I think that would be a lot of fun to see again.

Brazeal, Jason [22:52]: I agree, Vinnie, but I think people that aren't familiar with the prompting language should see the hello world prompts first, before they look at Agora.

Falco, Vinnie [23:02]: No, no, no, you misunderstand me. I don't wanna look at the prompt, I wanna look at the Reddit page.

Brazeal, Jason [23:08]: Oh, OK.

Falco, Vinnie [23:09]: I want them to see the output, cause it's fun, it's really fun. They should see it. It's exciting. The prompt is not exciting. I mean, it's exciting to me, but it's a computer program, so it is what it is. Everyone will choose one of these prompts. Obviously Will has Mentographist, because that's pretty much what Talktron is, and then these others are all kind of fairly equal.

Falco, Vinnie [23:45]: If anyone can't get a prompt cause there's not enough, then I'll find another one, or we'll come up with something else. This will be a long term project: every week you'll do a little bit, you're not gonna write the whole thing at once. But it's very important not to use any AI. You have to write it by hand, because if you use AI you're not gonna learn anything. It's writing it by hand and looking it up, and that's how you're gonna learn how it works.

Falco, Vinnie [24:13]: If you don't understand the prompt, of course I can help. What will end up happening is you'll probably get to a certain point and then there'll be a feature that's missing in Runvil, and then that will guide us and we'll say, maybe we'll prioritize that missing feature. Or if you have a workaround, maybe we add that feature and then you're able to make the prompt better.

Falco, Vinnie [24:33]: Some of these prompts will require a little custom harness that you'll have to write, a little web server, or a command line wrapper. I'll leave it up to you to figure out how you want to present it.

Falco, Vinnie [24:51]: Part of the format that I want to encourage is that there are two types of meetings: the meetings where I'm in it, and the meetings where I'm not in it. There will be a meeting where you all get together without me influencing you, and you can talk about what worked for you, what didn't work for you. If you wanna critique me, of course that's fine. I have a thick skin, I don't mind. Blow some steam, whatever.

Falco, Vinnie [25:24]: Any questions? Now's the time. Nothing? Come on.

Robles, Mateo [25:36]: I had one question. I thought your idea of the Knowledge Capture Corporation is really interesting, but what I was wondering is how much of the angle will go to consulting or process improvement. Let's say we do an interview and we capture some knowledge: is there a plan also to create tools to allow the enterprise to distribute that knowledge more effectively through software or anything of the sort, or is that out of scope?

Falco, Vinnie [26:12]: No, the pipeline starts with the capture and it ends when it's transferred to another person. The whole point of capturing the knowledge is so that you can transmit it to another individual. This is what's called the succession problem. If you have Steve Jobs, a great example, who comes after Steve Jobs? You have to train your replacement, but if you don't have that knowledge captured, then there's no way to train them.

Falco, Vinnie [26:47]: This happens in every organization where there's 2 people or more. You have somebody who knows something, and they're going away, and the knowledge goes with them. It's a problem that's as old as time. Now we have the AI tools, we have an opportunity to solve that problem, at least partially. We capture the knowledge from the people that are going away, we store it, and then we transmit it by creating an adaptive learning program.

Falco, Vinnie [27:13]: Right now that learning program is basically just a prompt that you run in Cursor or Claude Code, so that means it's just typing. But it doesn't need to be. If we're using Runvil, we could create a harness around it that's connected to a web server, and it looks like a textbook. We could deliver a page that looks like a textbook. It's got some text that comes from the model, and maybe it has a little box with a multiple choice. The model can do a tool call and say, do a quick quiz on this.

Falco, Vinnie [27:43]: Quiz him in order to move to the next section. If they fail, then the adaptive learning program will back off and try to teach them some more, and then it'll quiz them again. That could be graphical. That's all in the harness and the tool calls, and Runvil enables that. Does that answer your question?

Robles, Mateo [28:06]: Yeah, definitely, and I see the vision more clearly now. The value will kind of be in that transference from the individual to another individual, and not necessarily in any implemented workflow for the enterprise.

Falco, Vinnie [28:23]: That's right. Now I want you to notice what's happening. We have that exact problem right now, and I am tasking you all with the last step, with the knowledge transfer. I'll help, I'll participate in the capture by relating my stories of Runvil design.

Falco, Vinnie [28:45]: Then you all will figure out how to capture that from the transcript, put it into a format that's durable in this repository, and then your prompts will extract it and turn it into information that can be used with the learning program. At first you might just make an instruction manual that you can just read. That's useful. Then you might interact with the Pedagogue and figure out how to present that in an interactive way. This process that we're doing will be the prototype for the Knowledge Capture Corporation.

Falco, Vinnie [29:15]: What we learn about how you learn, what works for you, what doesn't work for you, we're gonna record all that, and that's gonna become the case study, that's gonna become our research that's gonna back the product. We're enacting the knowledge capture process.

Falco, Vinnie [29:33]: That's the realization that I had this morning: the problem that I'm trying to solve is, how do I transfer that knowledge that I have over to you all. If we can solve that, now we're in business. Now we're moving away from the nonprofit, now we're moving to the for-profit.

Falco, Vinnie [29:51]: Just to be clear, Runvil and Runvil Workshop will always be open source.

Elbaz, Jonathan [29:59]: Where will the knowledge capsule be stored?

Falco, Vinnie [30:00]: Can someone translate that for me, please?

Elbaz, Jonathan [30:07]: I just want to know where the knowledge capture will be stored in Runvil Classroom.

Elbaz, Jonathan [30:13]: And how it will be viewed.

Falco, Vinnie [30:18]: Are you asking me how are we gonna store the knowledge?

Elbaz, Jonathan [30:22]: Yes.

Falco, Vinnie [30:24]: Isn't it obvious? Markdown, it's all Markdown. Everything's Markdown.

Elbaz, Jonathan [30:31]: OK, but where? I'm looking at the Runvil Classroom repository, and I didn't see anything regarding the knowledge.

Falco, Vinnie [30:35]: We'll add a folder. Adding folders is free, don't worry.

Falco, Vinnie [30:51]: Let me restate it. I'm not pretending to have all the answers. What we're doing is an exploration that we're performing together. Don't think I have the answers to everything, because I don't. If I did, I would just say it. But I need your help exploring this space, cause the problem is bigger than one person. That's why I put together this team.

Robles, Mateo [31:16]: To go off the point Jonathan raised, and I'm not sure if this is the direction he was going, but I'm thinking in terms of some enterprises that might be more confidential or more privacy based in terms of their knowledge. Is that something you would consider, maybe splitting the service into a self-hosting and a service where we kind of service them also?

Falco, Vinnie [31:52]: I don't wanna know about it. I don't want to care about it. I only want us to solve the interesting problem. When we have the real prototype that works end to end, that's deployed and really solid, then I will bring in an investment partner.

Falco, Vinnie [32:10]: They'll get a fraction of ownership. What they're gonna provide is they're going to bring people who know how to deploy to the cloud, and the people who are going to solve that problem that you're talking about. That problem that you described is not interesting to me at all, because it's already been solved a million times, and I have nothing special to add to it, and it is also quite frankly busy work. I'd rather work on the cool part: the Pedagogue and the Mentographist and the AI talking to you, and then hopefully at some point we can have an animation, an avatar that talks to you like a person.

Falco, Vinnie [32:43]: Hopefully that technology will be where it needs to be. If you go to Live Avatar, you can chat with it and it's a virtual person. It looks very good. It's a little choppy over the internet. Will, you remember when we looked at that?

Pak, Will [33:15]: Yeah.

Falco, Vinnie [33:17]: By the time we're done with this, a year from now, those virtual avatars will be really good, and you'll be able to talk to them live. Anyway, did I answer your question?

Robles, Mateo [33:33]: Yeah, for sure.

Falco, Vinnie [33:35]: I see there's a CoC Blocker. Greg, it sounds like you wanna work on the CoC Blocker. Is that the one you want? It's fun, it's a fun little prompt. They're all fun.

Kaleka, Greg [33:47]: Sure, I'll take it.

Falco, Vinnie [33:57]: Now comes the hard part, which is we need to figure out how do we capture the knowledge. If we look at the SAGE paper, Saving All Gathered Expertise.

Falco, Vinnie [34:15]: The premise of this paper is that there are two people: the person who has the knowledge, and the interviewer. The person who's doing the interview has to have a certain skill: you get them to talk about the subject, you guide them to the subject, and you try to get them to tell a story. That's what the Mentographist does. The way this tool works is you start chatting.

Falco, Vinnie [34:46]: It's trying to find a story, it's trying to get you to lock onto a story, and as soon as it finds your story, then it performs what's called a story walk. It gets you to talk about it: it starts asking you questions about the story. Then when the story runs out, when the energy comes down, we go into the variations.

Falco, Vinnie [35:16]: Enrichment, and there's a mutation where we ask what if: the AI will say, hey, what if you'd made a different choice. That exploration of those what ifs, those modifications to the story, is the most revealing aspect, because it explains how the person thinks. So these are the variations.

Falco, Vinnie [35:40]: These are the seven ways that it can modify your story and ask you a question that makes you think about your story from a different angle. That capture is super valuable.

Falco, Vinnie [35:57]: I wanna do that with you, but where you're all asking me questions.

Falco, Vinnie [37:21]: There are some challenges that we need to solve. One of them is that the inference endpoint has multiple shapes. Right now, Gateway speaks what's called the OpenAI model. It's a generic inference endpoint where you present the messages and the tools, and then it gives you the response. However, if you wanna get the most from Claude, or you want to get the most from ChatGPT, or from some of the other frontier models, you have to use their special API, which is a little bit different. It's not the same, so you have to do something special.

Falco, Vinnie [38:15]: Gateway will speak a generic protocol, and then it will translate that into whatever Anthropic wants, and there will need to be a little bit of cooperation with the Runvil engine.

Falco, Vinnie [38:52]: Anthropic models send their thinking blocks with an additional cryptographic signature, so that you can't tamper with it, because if you were able to tamper with the thinking, then you could distill more effectively.

Falco, Vinnie [39:08]: Anyone who's interacting with the Anthropic endpoint, when they receive the messages, they have to also receive the little cryptographic hash that's attached to each of the thinking blocks, and they have to send it back. You're not allowed to reorder, you can't edit anything from the past cause it would invalidate the signature. You can't remove anything, you can't reorder anything, because then that would break the chain. If you break the chain, it kicks you out of the cache and it sends you an HTTP error. It won't accept the request.

Falco, Vinnie [39:42]: OpenAI ChatGPT have something even worse, which is they have encrypted thinking blocks that are redacted. The thinking block that comes back to you looks like random bits, and because they have the encryption key, it's the same thing: you have to send it back as is. If you tamper with it, then they reject your response.

Falco, Vinnie [40:04]: They also have different ways that they handle tools, little slight differences. On the Gateway side, Andrew, our goal is to make our endpoint support the generic API and add just enough extensions that other apps who use the Gateway won't be bothered, but Runvil will be able to use minimal extensions, so that the Gateway can translate those requests into the proprietary API.

Wheeler, Andrew [40:35]: Do we have that internal API worked out yet, or is that something we're gonna have to work out? Protocol?

Falco, Vinnie [40:45]: It's a great question.

Falco, Vinnie [40:54]: I'll say this, though: it's not that complicated.

Falco, Vinnie [40:59]: Since we're on that subject, let's talk about some technical stuff, and these are the things that we want to extract. This is the tacit knowledge. You're not gonna find this by looking in the Rust code, that's just gonna overwhelm you.

Falco, Vinnie [41:16]: I want to make sure this doesn't go over an hour, cause that could be too much for everyone. I want to make it like a normal high school class: 50 minutes, the bell.

Prevatte, Cynthia [41:33]: I'm tracking the time.

Falco, Vinnie [41:35]: We got about 9 minutes.

Falco, Vinnie [41:42]: At the 5 minute mark, I want to give everyone their assignments. If we look in the classroom at how an agent harness works, I'm gonna talk about this in terms of Runvil.

Falco, Vinnie [41:58]: The agent takes the chat, the whole history, and then it takes whatever you type in, and it packs that thing up. It sends it to the Gateway, and then the Gateway sends it out to Claude. Claude calculates its response, sends it back to the Gateway, and the Gateway sends it to the Workshop, and then that appears as additional chat. We have a little extension where we can stream the results, so you can see the words coming in. We don't want to wait until the whole thing comes in, that would be lame. We wanna see those words right away. We want that instant gratification.

Brazeal, Jason [42:36]: It's not just your message that gets sent though. There's a whole background prompt, a context, like chat.md.

Falco, Vinnie [42:39]: Yeah.

Brazeal, Jason [42:51]: The chat.md says something like you're running the harness, things like that, right?

Falco, Vinnie [42:56]: Yeah, right. That's part of the messages, but I don't want to overwhelm everyone.

Brazeal, Jason [42:59]: There's a system prompt.

Falco, Vinnie [43:02]: Yeah. The point is that there's a stack of messages that Runvil maintains, and it grows every time you add something to it. When we want to talk to the model, we send that whole package to the model, and then it sends us the response back.

Falco, Vinnie [43:19]: Then that conversation grows. Eventually, when it gets too big, we know what happens, we get what I call the AI lobotomy: it compacts, and then now you have room again. In Runvil, in the prompt, that collection of messages is called messages.

Falco, Vinnie [43:42]: If you look at a prompt, you can see it. Let me show you the prompt for the chat. I can remind you where it is: agents. This is the Runvil prompt that operates the chat.

Falco, Vinnie [43:58]: At the bottom, we get a message from the user, we get some input from that box, and we append it to the messages. This is a builder: it appends the text to the messages, so now messages is getting bigger, and then we run the tool loop. That's explained in the document that's in the lessons.

Falco, Vinnie [44:22]: This will send the new stack of messages. So, Andrew, to answer your question: on the Lua side, there's this object, but it's represented by something in Rust. That messages object is backed by a Rust struct.

Falco, Vinnie [44:39]: You can add to that. You can add little invisible stuff that the Lua can't see, so you could add what you need so that you could talk to the Gateway. For example, this carries the tool catalog. We're at the 5 minute mark.

Falco, Vinnie [44:58]: Here's what I would like to do for the next session. Cynthia is gonna organize the sessions. I would like everyone to take the transcript and make a prompt.

Falco, Vinnie [45:15]: Run your prompt on the transcript and save the result. Cynthia will put the transcript here. You make a prompt in your student folder, you run it on the transcript.

Falco, Vinnie [45:33]: You put the output in your directory, you can organize it however you want, and then submit a pull request. Then merge it yourself, but don't merge it with the command line, don't merge with the GitHub interface. That sound good?

Brazeal, Jason [45:48]: What's the evaluation criteria here? How do we know that we did a good job?

Falco, Vinnie [45:49]: I have no idea. We'll look at them together and we'll comment on what looks good or what doesn't, and we'll try to figure it out, cause I don't know.

Falco, Vinnie [46:06]: I don't have all the answers. I wrote Scribe. I don't know how good Scribe will be. I'll run Scribe, and you can compare. You could probably do better than Scribe, because you'll make an extraction that's specific. You'll say, anything about Runvil, put that first. Scribe doesn't do that.

Brazeal, Jason [46:30]: You want it to be only based on the transcript. It can't look at the code or anything else, right?

Falco, Vinnie [46:37]: Yeah, just the transcript is fine. We don't wanna go into the weeds. We don't wanna have a prompt that takes too long to run. This is a learning experience for everyone to learn how to write reusable prompts, and it's also a group experience where we can learn.

Falco, Vinnie [46:53]: Given a transcript, which represents an interview, how do we extract the most important information and store it in a way that can be taught? That's the core problem, and you're all gonna try to solve it your own way.

Falco, Vinnie [47:11]: Then we're gonna see what works, and we're gonna work together, and in a week or two, after we've done this several times, I think we're gonna have some pretty good prompts, cause it'll get better each time. I would like everyone to choose one of these tools. Will gets Mentographist. Jason and Glenn get Agora. You all can fight over the things that are left. If two people have to grab the same prompt, that's OK; just pick the one that you like.

Falco, Vinnie [47:47]: Any questions? OK, this was a lot of fun. Cynthia will organize the rest.

Prevatte, Cynthia [48:00]: I will. The first thing I'll do is put the transcript so everyone can access it.

Falco, Vinnie [48:02]: Sounds really good.
