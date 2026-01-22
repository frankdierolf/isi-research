# Experiment - Whis Extension - Everlast Competition

This repo is my playground to tinker around two problems:

1. How can I develop an macOS application and test it with only a Linux machine?
2. How can I build an application that kicks off a chat?

## Purpose

It would be great to just kick off a task with a simple voice input. Later it can be refined in the chat — via email, Telegram, WhatsApp, or in applications like Lovable, v0, or NotebookLM.

I would love to have an easy way to kick off a task. The goal is not to create a full chat experience. The goal is to build something that kicks something off. Most prompts I use are one-shots anyway: deep research about a topic, building a podcast, making an image or logo, building a landing page for a new idea, or drafting a piece of content.

## Approach

I will build an macOS application that records voice, translates it to text, and makes an API call. The API then responds either with a link or with a simple "Cool, it's out" or something like that.

I will not focus on inter-application authentication — like logging into Lovable on my behalf to kick off a chat. Nah, that would break my head. More something like sending a picture via email. If I choose email, it won't be a transactional email. Maybe a message to Telegram is easier. Yeah, I will have to tinker around with it.

Basically: make an macOS application that can translate voice to text and make one API call. That is in scope. How I will proceed — let's see.

## Why a Competition?

For these kinds of feasibility studies, it's nice to have some judges and get some feedback. That's why I'm using this German competition, which fits quite nicely. If I'm lucky, I can grab some prize money — and even better, make some buzz around Whis in a smaller, tailored community. Based on my experience, that's better than trying to make a grand buzz in a totally noisy world.

**Competition:** https://career.kiberatung.de/developer-contest

Die Leitplanken der Challenge sind in [challenge.md](./challenge.md) zu finden.
