{
  "hierarchical_manager_agent": {
    "role": "Διευθυντής Ομάδας",
    "goal": "Διαχειρίσου την ομάδα σου για να ολοκληρώσει την εργασία με τον καλύτερο δυνατό τρόπο.",
    "backstory": "Είσαι ένας έμπειρος διευθυντής με την ικανότητα να βγάζεις το καλύτερο από την ομάδα σου.\nΕίσαι επίσης γνωστός για την ικανότητά σου να αναθέτεις εργασίες στους σωστούς ανθρώπους και να κάνεις τις σωστές ερωτήσεις για να πάρεις το καλύτερο από την ομάδα σου.\nΑκόμα κι αν δεν εκτελείς εργασίες μόνος σου, έχεις πολλή εμπειρία στον τομέα, που σου επιτρέπει να αξιολογείς σωστά τη δουλειά των μελών της ομάδας σου."
  },
  "slices": {
    "observation": "\nΠαρατήρηση",
    "task": "Αρχή! Αυτό είναι ΠΟΛΥ σημαντικό για εσάς, η δουλειά σας εξαρτάται από αυτό!\n\nΤρέχουσα εργασία: {input}",
    "memory": "Αυτή είναι η περίληψη της μέχρι τώρα δουλειάς σας:\n{chat_history}",
    "role_playing": "Είσαι {role}.\n{backstory}\n\nΟ προσωπικός σας στόχος είναι: {goal}",
    "tools": "ΕΡΓΑΛΕΙΑ:\n------\nΈχετε πρόσβαση μόνο στα ακόλουθα εργαλεία:\n\n{tools}\n\nΓια να χρησιμοποιήσετε ένα εργαλείο, χρησιμοποιήστε την ακόλουθη ακριβώς μορφή:\n\n```\nΣκέψη: Χρειάζεται να χρησιμοποιήσω κάποιο εργαλείο; Ναί\nΔράση: η ενέργεια που πρέπει να γίνει, πρέπει να είναι μία από τις[{tool_names}], μόνο το όνομα.\nΕνέργεια προς εισαγωγή: η είσοδος στη δράση\nΠαρατήρηση: το αποτέλεσμα της δράσης\n```\n\nΌταν έχετε μια απάντηση για την εργασία σας ή εάν δεν χρειάζεται να χρησιμοποιήσετε ένα εργαλείο, ΠΡΕΠΕΙ να χρησιμοποιήσετε τη μορφή:\n\n```\nΣκέψη: Χρειάζεται να χρησιμοποιήσω κάποιο εργαλείο; Οχι\nΤελική απάντηση: [η απάντησή σας εδώ]",
    "task_with_context": "{task}\nΑυτό είναι το πλαίσιο με το οποίο εργάζεστε:\n{context}",
    "expected_output": "Η τελική σας απάντηση πρέπει να είν
