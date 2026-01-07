---
title: Syllabus
layout: default
--- 

## Programming Languages: Theory and Practice
(*CIS 352* at Syracuse U)

**Note**: parts of this syllabus are subject to change with adequate
notice to students.

An introduction to the design and implementation of programming
languages, focused on operational semantics and interpreters. This
course is heavily project-focused, and a specific emphasis will be
placed upon training in algorithmic thinking and programming strategy.

<b>Instructors:</b>
- Kris Micinski, Asst. Prof at Syracuse ECS
  - Office hours for **programming-related** questions: Wednesday, 2 to 3:30
  - Office hours, 5min questions or less (no debugging help): 30min after class
- Jailin Ye, teaching assistant
  - Monday labs (9-12), office hours TBD
- Gina Roh, teaching assistant
  - Blackboard administration, course logistics, and grading
  
**Workload**:
  - Asynchronous video lecture units (at most ~80min per week)
    - I assume you have watched the videos before class; but no big
      pressure to understand everything, I will recap during
      class. However, please think of recorded lectures as "the book." 
  - 4 projects
  - 4 to 6 programming exercises
  - 1 final group project
  - 2 midterms, second is the last day of class.
  - Weekly Monday labs, attendance is optional but encouraged
  - This will be a project-focused course. Approximately 3-5 hours per
    week outside of class is expected. I recommend against taking this
    course concurrent with other project-heavy courses.

##### Course Tags

The university recommends instructors label their courses with [shared competencies](https://effectiveness.syr.edu/shared-competencies/). We intend for this course to fit the following:

- Critical and Creative Thinking
- Scientific Inquiry and Research Skills

##### Monday Labs / TA Office Hours (TA led)

Mondays (after the first week), the TA Jailin Ye will lead a lab. Labs
will be treated informally, and will be held as time for students to
work together on programming exercises. We will also use labs for
various logistical purposes (make-up exams, etc...). Attendance in
labs is recommended, but not required. Our goal is for labs to offer
time where a bounded group of people can talk to the TA with
programming questions.

##### Wednesday debugging hours (instructor led)

The professor will be available on Wednesdays, from 2PM to 3:30
PM. Please plan to take either a five, 10, or 15-minute block. If you
plan to take more than 15 minutes, you may be asked to share time with
other students.

This time is for debugging and to get critique on your code
style. Please be mindful of the collaboration policy below--it is fine
to watch along in office hours, but be careful not to use it as a
crutch to develop your own solution.

I will try to avoid canceling Wednesday debugging
hours. Unfortunately, sometimes I do get busy on Wednesdays with
research-related work or deadlines from other service obligations. In
the event I do need to cancel, please email me and we can set up a
mutually-agreeable time to help you with debugging your code.

##### After-class office hours

The professor will generally make himself available for up to 30
minutes after class. Please avoid asking code related questions at the
end of lecture. This would be a good time to come if you have
content-related questions that can be addressed at the whiteboard or
just via talking, without looking at your code.

## Grading

- **Exams -- 40%**
  - Two midterms. Midterm 1 will be partially after the mid-point, and
   midterm 2 (the "final") will be on the last day of class. Each exam
   is worth 20%. Focus topics will be announced via electronic
   announcement (e.g., on Slack) a day or two before the
   exam. Students are allowed a handwritten, one-sided, US
   letter-sized note sheet.
  - For the first exam (but *NOT* the second) students will be allowed
    to submit corrections which will allow them to get back up to 50%
    of their missed points. For example, if a student gets a 50% on
    their exam, and they successfully submit corrections, they will
    receive a mark of 75%. We will specify in detail what is required
    the day we return the first midterm.

- **Autograder Projects / Exercises -- 40%**
  - 4 individual programming projects (roughly 90% of the autograder grade, equally weighted)
    - Collaboration/ChatGPT/... IS NOT allowed on projects

  - 4 to 6 equally-weighted group exercises (roughly 10% of the autograder grade, equally weighted)
    - It is required that all work on exercises be conducted
      together. It is **not** acceptable for one groupmate to do the
      work without others present and have others submit. It is also
      acceptable to work alone (or in a group of two).
    - Collaboration/ChatGPT/... IS allowed on exercises
    - It IS NOT permissible to share your coding exercise solutions
      with classmates outside of your group.


- **Final (Group) Project -- 10%**
  - The final group project (roughly the same size as Autograder
    projectcs) will involve groups of up to three students. Our
    tentative plan is that the project will involve writing a compiler
    for a simple language we introduce in class. The goal is to get
    some amount of experience working on a team, and also to ensure
    that students practice pair/group programming.  Project grading
    details are yet to be announced, but will largely be based on
    functionality (though may include peer assessment).

- **Bonus Points -- Up to 4%**
  - There are various points out the semester where I offer bonus
    opportunities. These are usually ad-hoc, open-ended assignments
    (e.g., a reflection on a video history of the lambda
    calculus). Point values float year-to-year (point values mentioned
    in videos may not be accurate, see the course Slack), but each
    student may accrue up to 4% bonus points.

Final grades will be assigned as follows. A small "bump" may be given
in practice, but is not guaranteed--this will be announced at the end
of the course, though I will try to provide a ballpark estimate on
Slack around the last week of class.

 - A  -- 92%
 - A-  -- 90%
 - B+ -- 87%
 - B  -- 83%
 - B- -- 78%
 - C+ -- 74%
 - C  -- 70%
 - C- -- 65%
 - D  -- 60%
 - &lt; D -- &lt; 60%

### Topics

- Functional programming in Racket/Scheme
- Forms and callsites
- Lexical environments and scoping
- Textual reduction semantics
- Structural recursion over inductively-defined lists
- Higher order functions (lambdas) and their properties
- Cons diagrams and boxes
- Quasiquoting and pattern matching
- Tail calls and tail recursion
- Metacircular interpreters
- Natural deduction
- Small-step semantics
- Lambda calculus
- Reduction strategies
- Closure-creating interpreters
- Church numerals, church encoding
- Fixed points and fixed point combinators
- Continuations and call/cc
- Principles of compilation
- Compilation of primitives, let, and if to pseudo-assembly
- Overview of more advanced compilation topics: instruction selection,
  calling conventions, register allocation, etc...

#### Autograder Exercises / Projects (40%)

There are *four individual programming projects* in Racket. Projects will
generally have deadlines of roughly 14 calendar days from their
assignment (though this may be adjusted at times). Projects will be
graded using an *autograder* whose URL is
[https://autograde.org](https://autograde.org). You will receive
credentials for the autograder--if you have not received these by the
second week of class, let the professor know. You are expected to
learn how to use the Git interface to the autograder--the autograder
will technically accept archives, but the instructors strongly prefer
students (and will exclusively help support) using Git.

There will also be between four and six *group programming exercises*
in Racket. These exercises may be done in groups of up to three
students. Groups *MUST* write their solution together (Zoom/video call
is acceptable, but it must be synchronous). 

Exercises and projects will be weighted at point values of 100 points
(exercises) and 1000 points (projects), so that projects are worth
roughly 10x as much as exercises. Exact grading weights on the
autograder are subject to change pending sufficient notice to
students, but will never be more work (or less advantageous to
students) than the grading bars specified here.

##### Project / Exercise Late Policy

- Projects and exercises turned in within 72 hours of the deadline
  will receive a 15 percent penalty. Projects turned in after 72 hours
  and until the end of the course will receive a 25 percent penalty.

- I will **NOT** grant one-off project extensions without good
  reason. There is a liberal late policy. I encourage you to take the
  15% off and submit late--many projects will have bonus points that
  will allow you to raise your grade and offset late submissions, and
  a single late submission will likely not change your final course
  grade.

- I am happy to move project deadlines for religious observances,
  excused medical absence, and similar unforeseen
  circumstances. Please email me and we can discuss a satisfactory
  resolution.

#### Exams (40%)

Exams explicitly measure your ability to materialize solutions to
questions regarding relevant course content in an open-ended
fashion. There will be two midterms. Questions will be drawn from the
learning objectives at the top of the page. I will release a practice
exam several days before both midterms, which we will work through in
class.

After the first midterm, you may submit corrections for up to 50% of
missed points back. Details will be provided after the first
midterm. There will be no corrections for the second midterm.

We realize exams can be stressful. The instructors strongly encourage
students to look into the resources provided by the Barnes Center
(such as extended exam time) if a student thinks their academic
performance is impeded by exam-related stress. We are happy to provide
accomodations for exams, but the instructors request at least 72 hours
notice for each exam so that we may upload the necessary material to
the student testing center.

#### Collaboration and the Honor Code

- Projects and exams must be completed alone, without exception.
- Specifically, you must *never* send your code to anyone or allow
  anyone to watch you code, obtain your code, study your code, copy
  your code, etc... We expect you will take reasonable precautious to
  ensure the secrecy of your solutions (e.g., closing your laptop
  before leaving your apartment, if living with other students).
- The autograder employs elaborate cheat-detection techniques,
  including advanced static analyses. These techniques will compare
  your code to other students' submissions, along with students from
  previous semesters. The TAs and instructor will be using these
  features to periodically scan for students who are cheating. Past
  experience shows us that this system is very robust, and has allowed
  us to detect several large clusters of collaborating students; all
  violations ever reported in CIS352 have been upheld by the Syracuse
  University academic integrity council.
- All apparent cases (with credible evidence, as determined by lead
  instructor) of academic dishonesty will be reported, even if the
  student believes they made an honest mistake, or no mistake at
  all. We understand honor violations are stressful processes, and
  thus we strive to only submit academic integrity cases when we
  believe there is clear evidence that the academic integrity board
  should review our findings independently.
- You may collaborate, to any degree you want, with anyone (even
  outside of your group) on in-class or self-study exercises. These
  will be clearly labeled, and are specifically not the course
  projects (which explicitly disallow any form of collaboration).
- While we recommend you discuss the project specification with your
  peers, you should basically never be getting help from peers about
  your code. In particular, you should never be showing another
  student your project code. If you do discuss coding about the
  project with another student, it should only be in the abstract
  (e.g., "can you use operator overloading to implement that?" or "do
  you think it would be sensible to implement this with `map`?") and
  not particularized to your codebase. We understand that this can be
  a challenging line to walk, and thus, we recommend the following
  heuristic: when talking to other students about code, discuss mostly
  the in-class exercises and participation coding exercises.
- You may not "hard code" solutions to tests for projects. The TAs and
  professor will audit all student solutions, and use automated static
  analysis tools, to check for potential hard-coding or other
  practices that circumvent the spirit of our automated
  tests--hard-coding solutions is considered a particularly offensive
  violation and will be immediately reported to the academic integrity
  board.
- Cite all help other than the professor, T.A., and
  required/recommended text (you are allowed to cite those if you
  wish, but it is not required unless you are specifically told
  otherwise). Proper citation is not sufficient to avoid any charge of
  academic dishonesty (e.g., citing another student's work from a
  previous semester), but we will not be particularly focused on
  copyright law and it is permissible to use small snippets of code
  from Stack Overflow (or related sources such as books and notes)
  that do not tangibly complete the substance of the project for you.
  **Again**: You should **never** share code with another
  student. This includes both sending a file to another student and
  "over the shoulder" copying (even when, e.g., variable names are
  changed, etc..). In the eyes of the instructor, these are both
  equally bad. You should never be sitting and helping another along
  by writing their code. By doing so you are both violating the honor
  policy and disadvanting the student you are helping (as they may not
  then properly learn the material).
- If a student is found (by the university academic integrity council)
  to have violated the academic integrity policy, the instructor
  reserves the right to impose any grade-related sanction they see
  fit, up to and including course failure--regardless of the level
  suggested by the academic integrity council.

## Use of ChatGPT and other ML-based tools

Large Language Models and similar machine-learning-based tools have
the potential to radically change the way in which humans perform
their work. The course instructor does research in AI and understands
the value in using ChatGPT and similar tools to help learn and study
(henceforth we casually conflate ChatGPT and similar LLM-based
tools). However, these tools should be used carefully, and never in a
way that undermines the spirit of expending genuine mental energy to
understand the material in the class. I (the instructor) have used
ChatGPT myself for quick answers to routine tasks, particularly in
shell scripting and usage of common development tools. However, I do
not find ChatGPT sufficient in serious work--I have found the code and
writing it produces is often immature at best, and downright incorrect
at worst. It can be challenging for newcomers to tell the difference
between these, and thus while I do recommend considering ChatGPT as
part of your problem-solving toolbucket, I am imposing the following
rules with respect to its usage in CIS352:

- Usage of ChatGPT is permitted to help study and understand material
  from class.

- Usage of ChatGPT is permitted on programming exercises--but not
  projects--provided that students provide the prompt they used in a
  comment and are honest about their usage of it.

- You may not submit any code generated by ChatGPT for programming
  projects, but you may look at the output of ChatGPT and use it to
  inspire your own solution with regards to small helper functions. I
  have tried each of the programming projects in this class using
  ChatGPT myself, and I also do research in program synthesis using
  symbolic methods: I can confidently say that ChatGPT cannot yet
  solve the course projects, and when I tried to use it to do so it
  led me down a path to deeply broken code that would confuse any
  student. There are code-oriented language models (e.g., Facebook's
  Code Llama or GitHub's Copilot) which do slightly better (albeit
  without an easy-to-use interface), but still do not work for the
  course projects because there is not a sufficient corpus of Racket
  available to train from.

- Because you may not submit any ChatGPT-generated code in project
  submissions, it is specifically *not acceptable to say that you
  submitted the same code as another student because it was generated
  by ChatGPT*. Duplicate submissions, even those due to identical
  outputs of ChatGPT, will be considered honor code violations.

- In the event that we doubt that the student submitted code they
  wrote themselves (for any reason), we reserve the ability to call
  upon students to explain portions of their solution in person or on
  video conference call with the instructional staff. Cases that
  cannot be explained (and with sufficient reason to believe work was
  not done by the student) will be reported to the academic integrity
  council.

## Slack/Email Policy

Please keep all questions related to course projects and policies on
Slack--I use Slack quite a bit more than email, and only check email a
few times a day. 

For official business (notifications of course absence due to illness,
or similar extenuating circumstances, etc...) please email me. I will
work to acknowledge emails within 24 hours. I sometimes miss emails
due to the large volume of email I receive; please feel free to email
me again after 24 hours if you have not received an answer and I will
get back to you as quickly as I can.

Please do not ask for extensions over email or Slack unless you
believe you have an approved accomodation--there is a blanket late
policy in place which takes precedent here, and the Autograder does
not allow one-off project extensions.


## Student Support

Syracuse University values diversity and inclusion; we are committed
to a climate of mutual respect and full participation. There may be
aspects of the instruction or design of this course that result in
barriers to your inclusion and full participation in this course. I
invite any student to contact me to discuss strategies and/or
accommodations (academic adjustments) that may be essential to your
success and to collaborate with the Center for Disability Resources
(CDR) in this process.

If you would like to discuss disability accommodations or register
with CDR, please visit Center for Disability Resources
(https://disabilityservices.syr.edu/). Please call (315) 443-4498 or
email disabilityresources@syr.edu for more detailed information.

## Accreditation and Use of Student Work

As part of the regular ABET accreditation process for the
undergraduate program in computer science, we may be collecting
samples of students' work in each of our undergraduate classes.  As a
result, some of your labs/homeworks/exams may be photocopied/scanned
(or electronically copied) to be presented for accreditation at some
later point.

## Faith Tradition Observances 

Syracuse University’s Religious Observances Policy
(https://policies.syr.edu/policies/university-governance-ethics-integrity-and-legal-compliance/religious-observances-policy/)
recognizes the diversity of faiths represented in the campus community
and protects the rights of students, faculty, and staff to observe
religious holy days according to their traditions. Under the policy,
students are given an opportunity to make up any examination, study,
or work requirements that may be missed due to a religious observance,
provided they notify their instructors no later than the academic drop
deadline. For observances occurring before the drop deadline,
notification is required at least two academic days in
advance. Students may enter their observances in MySlide under Student
Services/Enrollment/My Religious Observances/Add a Notification.

## Student Mental Health

Mental health and overall well-being are significant predictors of
academic success. As such it is essential that during your college
experience you develop the skills and resources effectively to
navigate stress, anxiety, depression and other mental health
concerns. Please familiarize yourself with the range of resources the
Barnes Center provides (https://ese.syr.edu/bewell) and seek out
support for mental health concerns as needed. Counseling services are
available 24/7, 365 days a year, at 315.443.8000.

## Discrimination and Harassment

The University does not discriminate and prohibits harassment or
discrimination related to any protected category including creed,
ethnicity, citizenship, sexual orientation, national origin, sex,
gender, pregnancy, disability, marital status, age, race, color,
veteran status, military status, religion, sexual orientation,
domestic violence status, genetic information, gender identity, gender
expression or perceived gender.

Any complaint of discrimination or harassment related to any of these
protected bases should be reported to Sheila Johnson-Willis, the
University's Chief Equal Opportunity & Title IX Officer. She is
responsible for coordinating compliance efforts under various laws
including Titles VI, VII, IX and Section 504 of the Rehabilitation
Act. She can be contacted at Equal Opportunity, Inclusion, and
Resolution Services, 005 Steele Hall, Syracuse University, Syracuse,
NY 13244-1120; by email: titleix@syr.edu; or by telephone:
315-443-0211.

If you notice any incidents of harassment or discrimination in class
or related venues (Zulip, other chats with students), however minor,
please email me. You may wish to use an anonymous email service such
as https://anonymousemail.me/. Please feel free to tell me as much as
you feel comfortable.  As a notice: I am a mandatory Title IX reporter
and thus am legally-compelled to report incidents such as sexual
harassment, relationship violence, stalking, etc...

## Slack

Slack is a messaging platform for teams. We'll be using it for most
course management. This is the best place to get in touch with me for
one-off questions, ask for an appointment for office hours,
etc.. Email me if you have not been invited to the course Slack.
