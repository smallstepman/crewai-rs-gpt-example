use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct HierarchicalManagerAgent {
    role: String,
    goal: String,
    backstory: String,
}

#[derive(Serialize, Deserialize)]
struct Slices {
    observation: String,
    task: String,
    memory: String,
    role_playing: String,
    tools: String,
    task_with_context: String,
    expected_output: String,
}

#[derive(Serialize, Deserialize)]
struct Errors {
    used_too_many_tools: String,
    agent_tool_missing_param: String,
    agent_tool_unexsiting_coworker: String,
    task_repeated_usage: String,
}

#[derive(Serialize, Deserialize)]
struct Tools {
    delegate_work: String,
    ask_question: String,
}

#[derive(Serialize, Deserialize)]
struct Translation {
    hierarchical_manager_agent: HierarchicalManagerAgent,
    slices: Slices,
    errors: Errors,
    tools: Tools,
}

fn main() {
    let translation = Translation {
        hierarchical_manager_agent: HierarchicalManagerAgent {
            role: "Διευθυντής Ομάδας".to_string(),
            goal: "Διαχειρίσου την ομάδα σου για να ολοκληρώσει την εργασία με τον καλύτερο δυνατό τρόπο.".to_string(),
            backstory: "Είσαι ένας έμπειρος διευθυντής με την ικανότητα να βγάζεις το καλύτερο από την ομάδα σου.\nΕίσαι επίσης γνωστός για την ικανότητά σου να αναθέτεις εργασίες στους σωστούς ανθρώπους και να κάνεις τις σωστές ερωτήσεις για να πάρεις το καλύτερο από την ομάδα σου.\nΑκόμα κι αν δεν εκτελείς εργασίες μόνος σου, έχεις πολλή εμπειρία στον τομέα, που σου επιτρέπει να αξιολογείς σωστά τη δουλειά των μελών της ομάδας σου.".to_string(),
        },
        slices: Slices {
            observation: "\nΠαρατήρηση".to_string(),
            task: "Αρχή! Αυτό είναι ΠΟΛΥ σημαντικό για εσάς, η δουλειά σας εξαρτάται από αυτό!\n\nΤρέχουσα εργασία: {input}".to_string(),
            memory: "Αυτή είναι η περίληψη της μέχρι τώρα δουλειάς σας:\n{chat_history}".to_string(),
            role_playing: "Είσαι {role}.\n{backstory}\n\nΟ προσωπικός σας στόχος είναι: {goal}".to_string(),
            tools: "ΕΡΓΑΛΕΙΑ:\n------\nΈχετε πρόσβαση μόνο στα ακόλουθα εργαλεία:\n\n{tools}\n\nΓια να χρησιμοποιήσετε ένα εργαλείο, χρησιμοποιήστε την ακόλουθη ακριβώς μορφή:\n\n```\nΣκέψη: Χρειάζεται να χρησιμοποιήσω κάποιο εργαλείο; Ναί\nΔράση: η ενέργεια που πρέπει να γίνει, πρέπει να είναι μία από τις[{tool_names}], μόνο το όνομα.\nΕνέργεια προς εισαγωγή: η είσοδος στη δράση\nΠαρατήρηση: το αποτέλεσμα της δράσης\n```\n\nΌταν έχετε μια απάντηση για την εργασία σας ή εάν δεν χρειάζεται να χρησιμοποιήσετε ένα εργαλείο, ΠΡΕΠΕΙ να χρησιμοποιήσετε τη μορφή:\n\n```\nΣκέψη: Χρειάζεται να χρησιμοποιήσω κάποιο εργαλείο; Οχι\nΤελική απάντηση: [η απάντησή σας εδώ]".to_string(),
            task_with_context: "{task}\nΑυτό είναι το πλαίσιο με το οποίο εργάζεστε:\n{context}".to_string(),
            expected_output: "Η τελική σας απάντηση πρέπει να είναι: {expected_output}".to_string(),
        },
        errors: Errors {
            used_too_many_tools: "Έχω χρησιμοποιήσει πάρα πολλά εργαλεία για αυτήν την εργασία. Θα σας δώσω την απόλυτη ΚΑΛΥΤΕΡΗ τελική μου απάντηση τώρα και δεν θα χρησιμοποιήσω άλλα εργαλεία.".to_string(),
            agent_tool_missing_param: "\nΣφάλμα κατά την εκτέλεση του εργαλείου. Λείπουν ακριβώς 3 διαχωρισμένες τιμές σωλήνων (|). Για παράδειγμα, `coworker|task|context`. Πρέπει να φροντίσω να περάσω το πλαίσιο ως πλαίσιο.\n".to_string(),
            agent_tool_unexsiting_coworker: "\nΣφάλμα κατά την εκτέλεση του εργαλείου. Ο συνάδελφος που αναφέρεται στο Ενέργεια προς εισαγωγή δεν βρέθηκε, πρέπει να είναι μία από τις ακόλουθες επιλογές: {coworkers}.\n".to_string(),
            task_repeated_usage: "Μόλις χρησιμοποίησα το {tool} εργαλείο με είσοδο {tool_input}. Άρα ξέρω ήδη το αποτέλεσμα αυτού και δεν χρειάζεται να το χρησιμοποιήσω τώρα.\n".to_string(),
        },
        tools: Tools {
            delegate_work: "Χρήσιμο για την ανάθεση μιας συγκεκριμένης εργασίας σε έναν από τους παρακάτω συναδέλφους: {coworkers}.\nΗ είσοδος σε αυτό το εργαλείο θα πρέπει να είναι ένα κείμενο χωρισμένο σε σωλήνα (|) μήκους 3 (τρία), που αντιπροσωπεύει τον συνάδελφο στον οποίο θέλετε να του ζητήσετε (μία από τις επιλογές), την εργασία και όλο το πραγματικό πλαίσιο που έχετε για την εργασία .\nΓια παράδειγμα, `coworker|task|context`.".to_string(),
            ask_question: "Χρήσιμο για να κάνετε μια ερώτηση, γνώμη ή αποδοχή από τους παρακάτω συναδέλφους: {coworkers}.\nΗ είσοδος σε αυτό το εργαλείο θα πρέπει να είναι ένα κείμενο χωρισμένο σε σωλήνα (|) μήκους 3 (τρία), που αντιπροσωπεύει τον συνάδελφο στον οποίο θέλετε να το ρωτήσετε (μία από τις επιλογές), την ερώτηση και όλο το πραγματικό πλαίσιο που έχετε για την ερώτηση.\nΓια παράδειγμα, `coworker|question|context`.".to_string(),
        },
    };

    println!("{}", json!(translation).to_string());
}
