use tonic::{transport::Server, Request, Response, Status};
pub mod cat_judge {
    #[allow(clippy::all)]
    tonic::include_proto!("cat_judge");
}

use cat_judge::{
    executer_server::{Executer, ExecuterServer},
    ExecuteProblemReply, ExecuteProblemRequest,
};

mod usecase {
    use uuid::Uuid;
    type ProblemId = Uuid;
    type Language = String;
    type ExecutionId = Uuid;
    pub struct ExecutionStatus {
        pub execution_id: ExecutionId,
        pub message: String,
    }

    struct Executable {
        language: Language,
        file_path: String,
    }
    impl Executable {
        pub async fn new(
            execution_id: ExecutionId,
            language: Language,
            source_code: String,
        ) -> Result<Self, String> {
            Ok(Self {
                language: language.to_string(),
                file_path: format!("tmp/foo/{}.{}", execution_id, language),
            })
        }

        /// run with input from input_path and output into output_file
        pub async fn run_batch(
            &self,
            input_file: String,
            output_file: String,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    fn input_files(execution_id: Uuid) -> Vec<String> {
        vec!["aa".to_string(), "ii".to_string()]
    }

    pub async fn execute_problem(
        problem_id: ProblemId,
        language: Language,
        source_code: String,
    ) -> Result<ExecutionStatus, String> {
        let execution_id = Uuid::new_v4();
        let executable = Executable::new(execution_id, language, source_code).await?;
        for input_file in input_files(execution_id) {
            executable
                .run_batch(input_file, "/dev/null".to_string())
                .await?;
        }
        Ok(ExecutionStatus {
            execution_id,
            message: "Ok".to_string(),
        })
    }

    async fn save_file() -> Result<(), String> {
        Ok(())
    }
}

#[derive(Default)]
pub struct CatExecuter {}
use std::str::FromStr;
#[tonic::async_trait]
impl Executer for CatExecuter {
    async fn execute_problem(
        &self,
        request: Request<ExecuteProblemRequest>,
    ) -> Result<Response<ExecuteProblemReply>, Status> {
        let request = request.into_inner();
        let status = usecase::execute_problem(
            uuid::Uuid::from_str(&request.problem_id).unwrap(),
            request.language,
            request.source_code,
        )
        .await
        .unwrap();
        let reply = ExecuteProblemReply {
            execution_id: status.execution_id.to_string(),
            status: status.message,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:54321".parse().unwrap();
    let executer = CatExecuter::default();

    Server::builder()
        .add_service(ExecuterServer::new(executer))
        .serve(addr)
        .await?;

    Ok(())
}
