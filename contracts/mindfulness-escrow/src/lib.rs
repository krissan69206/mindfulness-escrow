#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct MindfulnessEscrow;

#[contractimpl]
impl MindfulnessEscrow {
    /// BƯỚC 1: Người dùng khóa token (XLM) vào quỹ để cam kết thói quen
    pub fn deposit(env: Env, user: Address, token: Address, amount: i128) {
        // Bắt buộc người dùng phải ký xác nhận giao dịch này
        user.require_auth(); 
        
        // Gọi contract của token để chuyển tiền từ ví người dùng vào contract này
        let client = token::Client::new(&env, &token);
        client.transfer(&user, &env.current_contract_address(), &amount);

        // Lưu trữ lại số tiền mà user này đã cam kết vào bộ nhớ của blockchain
        env.storage().persistent().set(&user, &amount);
    }

    /// BƯỚC 2A: Người dùng HOÀN THÀNH mục tiêu -> Admin xác nhận và trả lại tiền
    pub fn resolve_success(env: Env, admin: Address, user: Address, token: Address) {
        // Bắt buộc quyền quản trị (Admin/Hệ thống) để tránh việc user tự lấy lại tiền
        admin.require_auth(); 
        
        // Kiểm tra xem user này đã cọc bao nhiêu tiền
        let amount: i128 = env.storage().persistent().get(&user).unwrap_or(0);
        
        if amount > 0 {
            // Trả lại toàn bộ tiền từ contract về ví người dùng
            let client = token::Client::new(&env, &token);
            client.transfer(&env.current_contract_address(), &user, &amount);
            
            // Xóa bản ghi cam kết này để hoàn tất vòng đời
            env.storage().persistent().remove(&user);
        }
    }

    /// BƯỚC 2B: Người dùng THẤT BẠI -> Admin chuyển tiền cho tổ chức từ thiện
    pub fn resolve_fail(env: Env, admin: Address, user: Address, token: Address, charity: Address) {
        // Bắt buộc quyền quản trị (Admin/Hệ thống)
        admin.require_auth();
        
        // Lấy số tiền cọc của user
        let amount: i128 = env.storage().persistent().get(&user).unwrap_or(0);
        
        if amount > 0 {
            // Chuyển toàn bộ tiền cọc vào ví của quỹ từ thiện (charity)
            let client = token::Client::new(&env, &token);
            client.transfer(&env.current_contract_address(), &charity, &amount);
            
            // Xóa bản ghi cam kết
            env.storage().persistent().remove(&user);
        }
    }
}